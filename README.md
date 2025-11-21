# 🦀 FerrisFetch: Resilient Web Crawler & CLI

FerrisFetch is a robust **Rust**-based Web Crawler designed to download web pages and all their associated resources (CSS, JS, images) in a safe, efficient, and server-friendly way.

It uses advanced techniques such as **network resilience**, **retry with exponential backoff**, and **rate limiting** to avoid blocks and ensure successful downloads.

---

## ✨ Key Features

| Feature               | Description |
|-----------------------|-------------|
| **Full Download** | Downloads the main HTML and all linked assets (`<img>`, `<link>`, `<script>`). |
| **Network Resilience** | Failed requests are automatically retried using *Exponential Backoff*. |
| **Rate Limiting** | Configurable delay between downloads to prevent server overload. |
| **CLI Interface** | Accepts the target URL directly as an argument (e.g. `cargo run -- URL`). |
| **Styled Output** | Colorful logs, ASCII art, and tags like `[+]`, `[*]`, `[-]`. |

---

## 🛠️ How to Run the Tool

### Requirements

- **Rust** (installed via `rustup`)
- Build dependencies (Linux):

```bash
sudo apt install pkg-config libssl-dev
````

---

## 1. Clone and Enter the Repository

```bash
git clone https://github.com/NoHup-lgtm/FerrisFetch.git
cd FerrisFetch
```

---

## 2. Run via Command Line (CLI)

Use `cargo run --` followed by the full URL.
The `--` separator is required to pass the argument correctly.

### Example:

```bash
cargo run -- URL-TARGET
```

---

## 🔍 Example Output

```text
 ╔═╗╔═╗╦═╗╔═╗╦═╗╦ ╦╔═╗╦ ╦
 ║ ║╠═╝╠╦╝╠═╣╠╦╝║║║╠═╣╚╦╝
 ╚═╝╩  ╩╚═╩ ╩╩╚═╚╩╝╩ ╩ ╩ 
      >> FERRIS FETCH <<

[+] Attempting to crawl URL: URL-TARGET
[+] Found 48 unique assets/links for download.

[*] Downloading (1/48) [Main HTML] -> URL-TARGET
[+] Saved Main HTML to: downloads/index.html
[-] Waiting 2 seconds (base delay)...

[*] Downloading (2/48) [CSS] -> URL-TARGET
[+] Status SUCCESS: 200 OK
[+] Saved to: downloads/style.css
...
```

---

## ⚙️ Internal Configuration

You can adjust the crawler’s behavior in `src/main.rs`:

| Constant                 | Default | Description                                                           |
| ------------------------ | ------- | --------------------------------------------------------------------- |
| `MAX_RETRIES`            | **5**   | Maximum number of retry attempts after a download failure.            |
| `INITIAL_DELAY_SECONDS`  | **2**   | Delay (in seconds) between downloading each asset.                    |
| `RETRY_DELAY_MULTIPLIER` | **2**   | Multiplier applied when encountering HTTP 429 (e.g. 2s → 4s → 8s...). |

---

## 📄 License

This project is licensed under the **MIT License**.

**Author:** *NoHup-lgtm*
