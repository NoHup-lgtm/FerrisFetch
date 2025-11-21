/*!
 * @toolname: FerrisFetch
 * @version: 0.1.0
 * @author: NoHup-lgtm
 * @date: 2025-11-21
 * @license: MIT
 *
 * Resilient Web Crawler that downloads the main HTML and all associated assets (CSS, JS, Images, links)
 * from a given URL, utilizing exponential backoff logic to prevent rate-limiting.
 * Usage: cargo run -- <URL>
 */

use reqwest::{Client, StatusCode};
use tokio::time::{sleep, Duration};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use url::Url;
use scraper::{Html, Selector};
use colored::Colorize; 
use std::env;

const MAX_RETRIES: u32 = 5;
const INITIAL_DELAY_SECONDS: u64 = 2; 
const RETRY_DELAY_MULTIPLIER: u32 = 2; 

const FERRIS_FETCH_LOGO: &str = r#"
 ╔═╗╔═╗╦═╗╔═╗╦═╗╦ ╦╔═╗╦ ╦
 ║ ║╠═╝╠╦╝╠═╣╠╦╝║║║╠═╣╚╦╝
 ╚═╝╩  ╩╚═╩ ╩╩╚═╚╩╝╩ ╩ ╩ 
    >> FERRIS FETCH <<
"#;

const DEVELOPER_INFO: &str = "Developer: NoHup-lgtm";
const GITHUB_LINK: &str = "GitHub: https://github.com/NoHup-lgtm/FerrisFetch";


fn extract_assets(html_content: &str, base_url: &str) -> Vec<String> {
    let document = Html::parse_document(html_content);
    let base = match Url::parse(base_url) {
        Ok(u) => u,
        Err(_) => {
            eprintln!("{} Invalid base URL: {}", "[-]".red(), base_url);
            return Vec::new();
        }
    };
    let mut assets = Vec::new();

    let selectors = vec![
        ("a", "href"),          
        ("img", "src"),         
        ("link", "href"),      
        ("script", "src"),      
    ];

    for (tag, attr) in selectors {
        let selector = match Selector::parse(tag) {
            Ok(s) => s,
            Err(_) => continue, 
        };
        
        for element in document.select(&selector) {
            if let Some(href) = element.value().attr(attr) {
                if let Ok(absolute_url) = base.join(href) {
                    assets.push(absolute_url.to_string());
                }
            }
        }
    }
    assets
}

async fn download_asset(
    client: &Client,
    url: &str,
    downloads_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut current_delay = INITIAL_DELAY_SECONDS;
    let mut retries = 0;
    
    while retries < MAX_RETRIES {
        match client.get(url).send().await {
            Ok(response) => {
                let status = response.status();
                
                if status.is_success() {
                    println!("{} Success Status: {}", "[+]".green(), status);
                    
                    let bytes = response.bytes().await?;

                    let parsed_url = Url::parse(url)?;
                    let segments = parsed_url.path_segments().unwrap();
                    let file_name = segments.last().unwrap_or("index").to_string();

                    let file_path = downloads_dir.join(file_name);
                    let mut file = File::create(&file_path)?;
                    file.write_all(&bytes)?;

                    println!("{} Saved to: {}", "[+]".green(), file_path.display());
                    return Ok(());
                    
                } else if status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error() {
                    retries += 1;
                    eprintln!(
                        "{} ERROR {}: Rate limit hit. Attempt {} of {}. Waiting {}s...", 
                        "[-]".red(), status, retries, MAX_RETRIES, current_delay
                    );
                    
                    sleep(Duration::from_secs(current_delay)).await;
                    current_delay *= RETRY_DELAY_MULTIPLIER as u64; 
                    
                } else {
                    eprintln!("{} Fatal error downloading {}: Status {}", "[-]".red(), url, status);
                    return Err(format!("HTTP Error: {}", status).into());
                }
            }
            Err(e) => {
                retries += 1;
                eprintln!("{} Network error: {}. Attempt {} of {}.", "[-]".red(), e, retries, MAX_RETRIES);
                
                if retries < MAX_RETRIES {
                    sleep(Duration::from_secs(current_delay)).await;
                    current_delay *= RETRY_DELAY_MULTIPLIER as u64;
                } else {
                    return Err(format!("Final network failure: {}", e).into());
                }
            }
        }
    }
    
    Err(format!("Final failure: Could not download {} after {} attempts.", url, MAX_RETRIES).into())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("{}", FERRIS_FETCH_LOGO.yellow());
        println!("{}", DEVELOPER_INFO.cyan());
        println!("{} {}", "*".white(), GITHUB_LINK.blue());
        println!("\n{} ERROR: URL not provided!", "[-]".red().bold());
        println!("{} Correct Usage: {}", "[*]".cyan(), "cargo run -- https://example.com/".yellow().bold());
        return Ok(());
    }
    
    let target_url = &args[1];

    println!("{}", FERRIS_FETCH_LOGO.yellow());
    println!("{}", DEVELOPER_INFO.cyan());
    println!("{} {}", "*".white(), GITHUB_LINK.blue());
    println!(""); 

    println!("{} Setting Up Standard Client...", "[+]".green());
    
    let client = Client::builder()
        .user_agent("FerrisFetch-Full-Crawler-v0.1")
        .build()?;

    let downloads_dir = Path::new("downloads");
    
    if !downloads_dir.exists() {
        tokio::fs::create_dir_all(&downloads_dir).await?;
        println!("{} Directory created in: {}", "[+]".green(), downloads_dir.display());
    }
    
    println!("{} Attempting to crawl URL: {}", "[+]".green(), target_url);

    let html_response = match client.get(target_url).send().await {
        Ok(r) => r.text().await?,
        Err(e) => {
            eprintln!("{} FAILED to download main HTML: {}", "[-]".red(), e);
            return Err(e.into());
        }
    };

    let assets_to_download = extract_assets(&html_response, target_url);

    let mut unique_assets: Vec<String> = vec![target_url.to_string()];

    for asset in assets_to_download.into_iter() {
        if !unique_assets.contains(&asset) {
            unique_assets.push(asset);
        }
    }

    println!("{} Found {} unique assets/links for download.", "[+]".green(), unique_assets.len());

    for (index, url) in unique_assets.iter().enumerate() {

        let file_type = if url.ends_with(".css") { "CSS" }
                        else if url.ends_with(".js") { "JS" }
                        else if url.ends_with(".png") || url.ends_with(".jpg") || url.ends_with(".svg") { "IMAGE" }
                        else if index == 0 { "Main HTML" }
                        else { "LINK" };

        println!("\n{} Downloading ({}/{}) [{}] -> {}", 
            "[*]".cyan(), index + 1, unique_assets.len(), file_type, url
        );

        if index == 0 {
            let file_path = downloads_dir.join("index.html");
            let mut file = File::create(&file_path)?;
            file.write_all(html_response.as_bytes())?;
            println!("{} Saved Main HTML to: {}", "[+]".green(), file_path.display());
            
        } else {

            match download_asset(&client, url, downloads_dir).await {
                Ok(_) => {}, 
                Err(e) => {
                    eprintln!("{} FAILED to download asset {}: {}", "[-]".red(), file_type, e);
                }
            }
        }

        if index < unique_assets.len() - 1 {
            println!("{} Waiting {} seconds (base delay)...", 
                "[-]".yellow(), INITIAL_DELAY_SECONDS
            );
            sleep(Duration::from_secs(INITIAL_DELAY_SECONDS)).await;
        }
    }
    
    println!("\n{} All asset downloads completed.", "[+]".green());
    Ok(())
}