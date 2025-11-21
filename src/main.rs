/*!
 *@toolname: RustDownloader
 *@version: 0.1.0
 * @author: [NoHup-lgtm]
 *@date: 2025-11-21
 *@license: MIT
 *
 * Script to track and download files from websites that are delayed between requests
 * and exponential backoff logic to prevent blocking by request limit (429 Too Many Requests).
 */

use reqwest::{Client, StatusCode};
use tokio::time::{sleep, Duration};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use url::Url;
use scraper::{Html, Selector};

const MAX_RETRIES: u32 = 5;
const INITIAL_DELAY_SECONDS: u64 = 2; 
const RETRY_DELAY_MULTIPLIER: u32 = 2; 
const TARGET_URL: &str = "URL_TARGET"; 

fn extract_links_from_html(html_content: &str, base_url: &str) -> Vec<String> {
    let document = Html::parse_document(html_content);
    let selector = Selector::parse("a").unwrap(); 
    let base = Url::parse(base_url).expect("invalid base url");
    let mut links = Vec::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(absolute_url) = base.join(href) {
                links.push(absolute_url.to_string());
            }
        }
    }
    links
}

async fn download_file(
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
                    println!("  -> success status: {}", status);
                    
                    let bytes = response.bytes().await?;
                    let file_name = Url::parse(url)?
                        .path_segments()
                        .and_then(|segments| segments.last())
                        .filter(|s| !s.is_empty())
                        .unwrap_or("index.html")
                        .to_string();

                    let file_path = downloads_dir.join(file_name);
                    let mut file = File::create(&file_path)?;
                    file.write_all(&bytes)?;

                    println!("  -> except in: {:?}", file_path);
                    return Ok(());
                    
                } else if status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error() {
                    retries += 1;
                    eprintln!(
                        "  -> ERROR {}: Attempt {} of {}. Waiting {}s...", 
                        status, retries, MAX_RETRIES, current_delay
                    );
                    
                    sleep(Duration::from_secs(current_delay)).await;
                    current_delay *= RETRY_DELAY_MULTIPLIER as u64; 
                    
                } else {
                    eprintln!(" -> Fatal error when downloading {}: Status {}", url, status);
                    return Err(format!("Error HTTP: {}", status).into());
                }
            }
            Err(e) => {
                retries += 1;
                eprintln!(" -> Network error: {}. Attempt {} of {}.", e, retries, MAX_RETRIES);
                
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
    let client = Client::new();
    let downloads_dir = Path::new("downloads");
    
    if !downloads_dir.exists() {
        tokio::fs::create_dir_all(&downloads_dir).await?;
        println!("Directorate created in: {:?}", downloads_dir);
    }
    
    println!("-- Step 1: Tracking the Initial URL: {} --", TARGET_URL);

    let html_response = client.get(TARGET_URL).send().await?.text().await?;

    let links_to_download = extract_links_from_html(&html_response, TARGET_URL);

    println!("-- Step 2: Found {} download links --", links_to_download.len());

    for (index, url) in links_to_download.iter().enumerate() {
        println!("\nRequesting ({}/{}) -> {}", index + 1, links_to_download.len(), url);

        match download_file(&client, url, downloads_dir).await {
            Ok(_) => {}, 
            Err(e) => {
                eprintln!("The download of {} failed irretrievably: {}", url, e);
            }
        }
        if index < links_to_download.len() - 1 {
            println!("\nWaiting {} seconds (base delay) before the next URL..", INITIAL_DELAY_SECONDS);
            sleep(Duration::from_secs(INITIAL_DELAY_SECONDS)).await;
        }
    }
    println!("\nDownload and tracking completed.");
    Ok(())
}