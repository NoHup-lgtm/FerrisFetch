# 🦀 FerrisFetch: Resilient & Concurrent Web Crawler

FerrisFetch is a robust **Rust**-based Web Crawler designed to download web pages and all their associated resources (CSS, JS, images) in a safe, efficient, and server-friendly way.

Now updated to version **1.0**, it uses advanced techniques such as **massive concurrency**, **stealth mode**, and **smart retries** to ensure successful downloads even faster.

---

## ✨ Key Features

| Feature               | Description |
|-----------------------|-------------|
| **Massive Concurrency** | Downloads hundreds of assets simultaneously using async streams (`-t` flag). |
| **Stealth Mode** | Automatically rotates User-Agents (Chrome, Firefox, Safari) to bypass basic WAFs. |
| **Full Download** | Downloads the main HTML and all linked assets (`<img>`, `<link>`, `<script>`, `<video>`). |
| **Network Resilience** | Failed requests are automatically retried using *Exponential Backoff*. |
| **Smart CLI** | Advanced arguments for output folder (`-o`) and thread count (`-t`). |
| **Visual Feedback** | Real-time progress bar showing download speed and status. |

---

## 🛠️ How to Run the Tool

### Requirements

- **Rust** (installed via `rustup`)
- Build dependencies (Linux):

```bash
sudo apt install pkg-config libssl-dev
````

-----

## 1\. Clone and Enter the Repository

```bash
git clone [https://github.com/NoHup-lgtm/FerrisFetch.git](https://github.com/NoHup-lgtm/FerrisFetch.git)
cd FerrisFetch
```

-----

## 2\. Run via Command Line (CLI)

Use `cargo run --` followed by the URL. You can now specify options\!

### Basic Usage:

Downloads to the default `downloads/` folder using 50 threads.

```bash
cargo run -- reidoscoins.com.br
```

### Turbo Mode (Advanced):

Specify a custom output folder and increase threads for maximum speed.

```bash
cargo run --release -- [https://example.com](https://example.com) -o my_dump -t 100
```

**Options:**

  - `-o`: Output directory name.
  - `-t`: Number of concurrent threads (default: 50).

-----

## 🔍 Example Output

```text
 ╔═╗╔═╗╦═╗╔═╗╦═╗╦ ╦╔═╗╦ ╦
 ║ ║╠═╝╠╦╝╠═╣╠╦╝║║║╠═╣╚╦╝
 ╚═╝╩  ╩╚═╩ ╩╩╚═╚╩╝╩ ╩ ╩ 
    >> FERRIS FETCH <<

[*] User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)...
[*] Analyzing: [https://reidoscoins.com.br](https://reidoscoins.com.br)
[+] Index saved.
[+] Found 142 unique assets.
[*] Downloading with 50 threads...

⠼ [########################################] 142/142 (0s)
[OK] Download finished in: downloads/
```

-----

## ⚙️ Internal Configuration

You can adjust the crawler’s behavior in `src/main.rs`:

| Constant                 | Default | Description                                                           |
| ------------------------ | ------- | --------------------------------------------------------------------- |
| `MAX_RETRIES`            | **5** | Maximum number of retry attempts after a download failure.            |
| `RETRY_DELAY_MULTIPLIER` | **2** | Multiplier applied when encountering errors (e.g. 2s → 4s → 8s...). |

-----

## 📄 License

This project is licensed under the **MIT License**.

**Author:** *NoHup-lgtm*
