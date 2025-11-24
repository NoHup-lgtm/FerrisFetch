/*!
 * @toolname: FerrisFetch
 * @version: 1.0.0
 * @author: NoHup-lgtm
 * @license: MIT
 */

use clap::Parser;
use colored::*;
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use rand::seq::SliceRandom; // Para escolher User-Agent
use reqwest::{Client, StatusCode};
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::time::sleep;
use url::Url;

const MAX_RETRIES: u32 = 5;
const RETRY_DELAY_MULTIPLIER: u32 = 2;

const FERRIS_FETCH_LOGO: &str = r#"
 ╔═╗╔═╗╦═╗╔═╗╦═╗╦ ╦╔═╗╦ ╦
 ║ ║╠═╝╠╦╝╠═╣╠╦╝║║║╠═╣╚╦╝
 ╚═╝╩  ╩╚═╩ ╩╩╚═╚╩╝╩ ╩ ╩ 
    >> FERRIS FETCH <<
"#;

#[derive(Parser, Debug)]
#[command(author = "NoHup-lgtm", version = "1.0.0", about = "FerrisFetch: Concurrent Asset Downloader")]
struct Args {
    /// A URL alvo (ex: https://site.com)
    #[arg(required = true)]
    url: String,

    /// Pasta de saída
    #[arg(short = 'o', long = "output", default_value = "downloads")]
    output: String,

    /// Número de threads simultâneas
    #[arg(short = 't', long = "threads", default_value_t = 50)]
    threads: usize,
}

fn get_random_user_agent() -> &'static str {
    let agents = [
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/121.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/121.0"
    ];
    agents.choose(&mut rand::thread_rng()).unwrap()
}

fn extract_assets(html_content: &str, base_url: &str) -> HashSet<String> {
    let document = Html::parse_document(html_content);
    let base = match Url::parse(base_url) {
        Ok(u) => u,
        Err(_) => return HashSet::new(),
    };
    
    let mut assets = HashSet::new();
    let selectors = vec![
        ("a", "href"), ("img", "src"), ("link", "href"), 
        ("script", "src"), ("video", "src"), ("source", "src"), ("audio", "src")
    ];

    for (tag, attr) in selectors {
        if let Ok(selector) = Selector::parse(tag) {
            for element in document.select(&selector) {
                if let Some(href) = element.value().attr(attr) {
                    if let Ok(absolute_url) = base.join(href) {
                        if absolute_url.scheme().starts_with("http") {
                            assets.insert(absolute_url.to_string());
                        }
                    }
                }
            }
        }
    }
    assets
}

async fn download_asset(client: Client, url: String, downloads_dir: PathBuf, pb: ProgressBar) {
    let parsed_url = match Url::parse(&url) {
        Ok(u) => u,
        Err(_) => return,
    };

    let file_name = parsed_url.path_segments()
        .map(|c| c.last().unwrap_or("index.html"))
        .unwrap_or("index.html");
    
    let safe_name = sanitize_filename::sanitize(file_name);
    let safe_name = if safe_name.is_empty() { "unknown_file".to_string() } else { safe_name };
    let file_path = downloads_dir.join(safe_name);

    if file_path.exists() {
        pb.inc(1); // Já existe, pula
        return;
    }

    let mut current_delay = 1;
    let mut retries = 0;

    while retries < MAX_RETRIES {
        match client.get(&url).send().await {
            Ok(response) => {
                let status = response.status();
                if status.is_success() {
                    if let Ok(bytes) = response.bytes().await {
                        let path_clone = file_path.clone();
                        let _ = tokio::task::spawn_blocking(move || {
                            if let Ok(mut file) = File::create(path_clone) {
                                let _ = file.write_all(&bytes);
                            }
                        }).await;
                        pb.inc(1);
                        return;
                    }
                } else if status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error() {
                    retries += 1;
                    sleep(Duration::from_secs(current_delay)).await;
                    current_delay *= RETRY_DELAY_MULTIPLIER as u64;
                } else {
                    pb.inc(1); // Erro 404/403, desiste
                    return;
                }
            },
            Err(_) => {
                retries += 1;
                sleep(Duration::from_secs(current_delay)).await;
                current_delay *= RETRY_DELAY_MULTIPLIER as u64;
            }
        }
    }
    pb.inc(1);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Args::parse();

    // Correção automática de URL
    if !args.url.starts_with("http") {
        args.url = format!("https://{}", args.url);
    }

    println!("{}", FERRIS_FETCH_LOGO.yellow());
    println!("Developer: NoHup-lgtm | Version: 1.0.0");
    
    let user_agent = get_random_user_agent();
    println!("{} User-Agent: {}", "[*]".cyan(), user_agent);

    let client = Client::builder()
        .user_agent(user_agent)
        .timeout(Duration::from_secs(15))
        .danger_accept_invalid_certs(true)
        .build()?;

    let downloads_dir = Path::new(&args.output);
    if !downloads_dir.exists() {
        tokio::fs::create_dir_all(downloads_dir).await?;
    }

    println!("{} Analisando: {}", "[*]".cyan(), args.url);

    let html_content = match client.get(&args.url).send().await {
        Ok(resp) => resp.text().await?,
        Err(e) => {
            eprintln!("{} Erro ao conectar: {}", "[-]".red(), e);
            return Ok(());
        }
    };

    let index_path = downloads_dir.join("index.html");
    let mut index_file = File::create(&index_path)?;
    index_file.write_all(html_content.as_bytes())?;
    println!("{} Index salvo.", "[+]".green());

    let mut assets = extract_assets(&html_content, &args.url);
    assets.remove(&args.url);

    let total_assets = assets.len() as u64;
    println!("{} Encontrados {} arquivos.", "[+]".green(), total_assets);

    if total_assets == 0 { return Ok(()); }

    println!("{} Baixando com {} threads...", "[*]".cyan(), args.threads);

    let pb = ProgressBar::new(total_assets);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    let fetch_tasks = stream::iter(assets)
        .map(|asset_url| {
            let client = client.clone();
            let dir = downloads_dir.to_path_buf();
            let pb = pb.clone();
            tokio::spawn(async move {
                download_asset(client, asset_url, dir, pb).await;
            })
        })
        .buffer_unordered(args.threads);

    fetch_tasks.collect::<Vec<_>>().await;

    pb.finish_with_message("Concluído!");
    println!("\n{} Download finalizado em: {}", "[OK]".green().bold(), downloads_dir.display());

    Ok(())
}