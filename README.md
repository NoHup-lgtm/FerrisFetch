# 🦀 FerrisFetch: Resilient & Concurrent Web Crawler

<div align="center">

```text
███████╗███████╗██████╗ ██████╗ ██╗███████╗    ███████╗███████╗████████╗ ██████╗██╗  ██╗
██╔════╝██╔════╝██╔══██╗██╔══██╗██║██╔════╝    ██╔════╝██╔════╝╚══██╔══╝██╔════╝██║  ██║
█████╗  █████╗  ██████╔╝██████╔╝██║███████╗    █████╗  █████╗     ██║   ██║     ███████║
██╔══╝  ██╔══╝  ██╔══██╗██╔══██╗██║╚════██║    ██╔══╝  ██╔══╝     ██║   ██║     ██╔══██║
██║     ███████╗██║  ██║██║  ██║██║███████║    ██║     ███████╗   ██║   ╚██████╗██║  ██║
╚═╝     ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚══════╝    ╚═╝     ╚══════╝   ╚═╝    ╚═════╝╚═╝  ╚═╝
````

**FerrisFetch** is a robust, high-performance **Rust**-based Web Crawler designed to download web pages and all their associated resources (CSS, JS, images, media) for offline analysis or static inspection.

Now featuring a **Professional Dashboard**, **Stealth Mode**, and **Massive Concurrency**.

\</div\>

-----

## ✨ Key Features

| Feature | Description |
| :--- | :--- |
| **Massive Concurrency** | Downloads hundreds of assets simultaneously using async streams (`-t` flag). |
| **Stealth Mode** | Automatically rotates User-Agents (Chrome, Firefox, Safari) to bypass basic WAFs. |
| **Modern Dashboard** | Clean, informative terminal UI showing Target, Config, and real-time status. |
| **Full Download** | Downloads the main HTML and all linked assets (`<img>`, `<link>`, `<script>`, `<video>`). |
| **Network Resilience** | Failed requests are automatically retried using *Exponential Backoff*. |
| **Smart CLI** | Advanced arguments for output folder (`-o`) and thread count (`-t`). |

-----

## 🛠️ Installation

### Requirements

  - **Rust** (installed via `rustup`)

### Build from Source

```bash
git clone [https://github.com/NoHup-lgtm/FerrisFetch.git](https://github.com/NoHup-lgtm/FerrisFetch.git)
cd FerrisFetch
cargo build --release
```

-----

## ⚡ Usage

### Basic Usage

Downloads to the default `downloads/` folder using 50 threads.

```bash
cargo run -- reidoscoins.com.br
```

### Turbo Mode (Advanced)

Specify a custom output folder (`-o`) and increase threads (`-t`) for maximum speed.

```bash
cargo run --release -- [https://example.com](https://example.com) -o my_dump -t 100
```

**Options:**

  - `-o`: Output directory name (Default: `downloads`).
  - `-t`: Number of concurrent threads (Default: `50`).

-----

## 🔍 Example Output

```text
███████╗███████╗██████╗ ... (Ferris Fetch Logo) ...
   v1.0.0 :: Developed by NoHup-lgtm
────────────────────────────────────────────────────────────
   🎯 TARGET:       [https://reidoscoins.com.br](https://reidoscoins.com.br)
   📂 OUTPUT:       downloads
   ⚡ THREADS:      50
   🕵️  STEALTH:      Mozilla/5.0 (Macintosh; Intel Mac...
────────────────────────────────────────────────────────────

[*] Connecting to target...
[+] Main HTML captured successfully.
[*] Extracting links and assets...
[+] Found 142 files. Starting massive download...

⠼ [████████████████████████████████████████] 142/142 (0s) Done!

════════════════════════════════════════════════════════════
   ✅ Download finished successfully!
   📂 Files saved in: downloads
════════════════════════════════════════════════════════════
```

-----

## ⚙️ Configuration

You can tweak the internal constants in `src/main.rs`:

| Constant | Default | Description |
| :--- | :--- | :--- |
| `MAX_RETRIES` | **5** | Max attempts per file before giving up. |
| `RETRY_DELAY_MULTIPLIER` | **2** | Multiplier for backoff (2s -\> 4s -\> 8s) on errors. |

-----

## 📄 License

This project is licensed under the **MIT License**.

**Author:** *NoHup-lgtm*
