/*!
 * @toolname: FerrisFetch
 * @version: 2.0.0
 * @author: NoHup-lgtm
 * @license: MIT
 */

use clap::Parser;
use colored::*;
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use rand::seq::SliceRandom;
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
███████╗███████╗██████╗ ██████╗ ██╗███████╗    ███████╗███████╗████████╗ ██████╗██╗  ██╗
██╔════╝██╔════╝██╔══██╗██╔══██╗██║██╔════╝    ██╔════╝██╔════╝╚══██╔══╝██╔════╝██║  ██║
█████╗  █████╗  ██████╔╝██████╔╝██║███████╗    █████╗  █████╗     ██║   ██║     ███████║
██╔══╝  ██╔══╝  ██╔══██╗██╔══██╗██║╚════██║    ██╔══╝  ██╔══╝     ██║   ██║     ██╔══██║
██║     ███████╗██║  ██║██║  ██║██║███████║    ██║     ███████╗   ██║   ╚██████╗██║  ██║
╚═╝     ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚══════╝    ╚═╝     ╚══════╝   ╚═╝    ╚═════╝╚═╝  ╚═╝
"#;

#[derive(Parser, Debug)]
#[command(author = "NoHup-lgtm", version = "1.0.0", about = "FerrisFetch: Concurrent Asset Downloader")]
struct Args {
    #[arg(required = true)]
    url: String,

    #[arg(short = 'o', long = "output", default_value = "downloads")]
    output: String,

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
        pb.inc(1); 
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
                    pb.inc(1); 
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

fn print_dashboard(url: &str, threads: usize, output: &str, ua: &str) {
    print!("\x1B[2J\x1B[1;1H"); 
    
    println!("{}", FERRIS_FETCH_LOGO.truecolor(255, 100, 0).bold()); // Laranja Rust
    println!("   {}", "v1.0.0 :: Developed by NoHup-lgtm".truecolor(150, 150, 150).italic());
    println!("{}", "────────────────────────────────────────────────────────────".truecolor(60, 60, 60));
    
    println!("   {:<15} {}", "🎯 TARGET:".bold().cyan(), url);
    println!("   {:<15} {}", "📂 OUTPUT:".bold().cyan(), output);
    println!("   {:<15} {}", "⚡ THREADS:".bold().cyan(), threads);
    println!("   {:<15} {}", "🕵️  STEALTH:".bold().cyan(), ua.chars().take(40).collect::<String>() + "...");
    
    println!("{}", "────────────────────────────────────────────────────────────\n".truecolor(60, 60, 60));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Args::parse();

    if !args.url.starts_with("http") {
        args.url = format!("https://{}", args.url);
    }

    // Seleciona User-Agent
    let user_agent = get_random_user_agent();

    // 1. Exibe o novo Dashboard Visual
    print_dashboard(&args.url, args.threads, &args.output, user_agent);

    let client = Client::builder()
        .user_agent(user_agent)
        .timeout(Duration::from_secs(15))
        .danger_accept_invalid_certs(true)
        .build()?;

    let downloads_dir = Path::new(&args.output);
    if !downloads_dir.exists() {
        tokio::fs::create_dir_all(downloads_dir).await?;
    }

    println!("{} Conectando ao alvo...", "[*]".yellow());

    let html_content = match client.get(&args.url).send().await {
        Ok(resp) => resp.text().await?,
        Err(e) => {
            eprintln!("{} Erro fatal ao conectar: {}", "[!]".red().bold(), e);
            return Ok(());
        }
    };

    let index_path = downloads_dir.join("index.html");
    let mut index_file = File::create(&index_path)?;
    index_file.write_all(html_content.as_bytes())?;
    println!("{} HTML principal capturado com sucesso.", "[+]".green());

    println!("{} Extraindo links e assets...", "[*]".yellow());
    let mut assets = extract_assets(&html_content, &args.url);
    assets.remove(&args.url);

    let total_assets = assets.len() as u64;
    
    if total_assets == 0 { 
        println!("{} Nenhum asset encontrado para baixar.", "[!]".yellow());
        return Ok(()); 
    }

    println!("{} Encontrados {} arquivos. Iniciando download massivo...", "[+]".green(), total_assets);

    let pb = ProgressBar::new(total_assets);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
        .unwrap()
        .progress_chars("█▓▒░  ")); 

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
    
    println!("\n{}", "════════════════════════════════════════════════════════════".truecolor(60, 60, 60));
    println!("   {} Download finalizado com sucesso!", "✅".green());
    println!("   {} Arquivos salvos em: {}", "📂".cyan(), downloads_dir.display().to_string().bold());
    println!("{}", "════════════════════════════════════════════════════════════".truecolor(60, 60, 60));

    Ok(())
}