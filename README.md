# 🦀 FerrisFetch: Web Crawler Resiliente e CLI

FerrisFetch é um Web Crawler robusto escrito em **Rust**, projetado para baixar páginas web e todos os seus recursos associados (CSS, JS, imagens) de forma segura, eficiente e educada com os servidores.

Ele utiliza técnicas avançadas de **resiliência**, **retry com exponential backoff** e **controle de taxa** para evitar bloqueios e garantir a conclusão dos downloads.

---

## ✨ Funcionalidades Principais

| Recurso               | Descrição |
|-----------------------|-----------|
| **Download Completo** | Baixa o HTML principal e todos os ativos vinculados (`<img>`, `<link>`, `<script>`). |
| **Resiliência de Rede** | Requisições falhas são repetidas utilizando *Exponential Backoff*. |
| **Controle de Taxa** | Atraso configurável entre downloads para evitar sobrecarregar servidores. |
| **Interface CLI** | Recebe a URL diretamente como argumento (ex: `cargo run -- URL`). |
| **Output Estilizado** | Logs coloridos, ASCII art e tags `[+]`, `[*]`, `[-]`. |

---

## 🛠️ Como Executar a Ferramenta

### Pré-requisitos

- **Rust** (instalado via `rustup`)
- Dependências de compilação (Linux):

bash
sudo apt install pkg-config libssl-dev

## 1. Clonar e Acessar o Repositório

bash
git clone https://github.com/NoHup-lgtm/FerrisFetch.git
cd FerrisFetch



## 2. Execução via Linha de Comando (CLI)

Use `cargo run --` seguido da URL completa.
O separador `--` é obrigatório para passar a URL ao programa.

### Exemplo:

bash
cargo run -- URL-TARGET/



## 🔍 Exemplo de Output

text
 ╔═╗╔═╗╦═╗╔═╗╦═╗╦ ╦╔═╗╦ ╦
 ║ ║╠═╝╠╦╝╠═╣╠╦╝║║║╠═╣╚╦╝
 ╚═╝╩  ╩╚═╩ ╩╩╚═╚╩╝╩ ╩ ╩ 
    >> FERRIS FETCH <<

[+] Attempting to crawl URL: https://www.reidoscoins.com.br/
[+] Found 48 unique assets/links for download.

[*] Downloading (1/48) [Main HTML] -> https://www.reidoscoins.com.br/
[+] Saved Main HTML to: downloads/index.html
[-] Waiting 2 seconds (base delay)...

[*] Downloading (2/48) [CSS] -> https://www.reidoscoins.com.br/style.css
[+] Status SUCESSO: 200 OK
[+] Saved to: downloads/style.css
...

---

## ⚙️ Configuração Interna

Ajuste os parâmetros do Crawler em `src/main.rs`:

| Constante                | Padrão | Descrição                                                          |
| ------------------------ | ------ | ------------------------------------------------------------------ |
| `MAX_RETRIES`            | **5**  | Máximo de tentativas de baixar um ativo após falha.                |
| `INITIAL_DELAY_SECONDS`  | **2**  | Tempo de espera entre cada download.                               |
| `RETRY_DELAY_MULTIPLIER` | **2**  | Multiplicador do atraso em caso de erro 429 (ex: 2s → 4s → 8s...). |


## 📄 Licença

Este projeto é licenciado sob a **MIT License**.

**Autor:** *NoHup-lgtm*
