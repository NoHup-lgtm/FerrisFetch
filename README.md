🦀 FerrisFetch: Web Crawler Resiliente e CLI

O FerrisFetch é um Web Crawler e downloader de ativos construído em Rust para ser rápido, robusto e amigável aos servidores. Ele automatiza o download de uma página web e seus recursos associados (CSS, JavaScript, Imagens), utilizando estratégias avançadas para garantir a conclusão do download sem ser bloqueado.

✨ Funcionalidades Principais

Recurso

Descrição

Download Completo

Faz o download do HTML principal e rastreia/baixa todos os ativos relacionados (<img>, <link>, <script>) para replicar a página localmente.

Resiliência de Rede

Implementa Backoff Exponencial para tentar novamente requisições falhas (erros de rede ou código 429 Too Many Requests), com tempo de espera crescente.

Controle de Taxa

Aplica um atraso base (INITIAL_DELAY_SECONDS) entre o download de cada arquivo para evitar sobrecarregar o servidor.

Linha de Comando (CLI)

Aceita a URL de destino diretamente como argumento de execução, facilitando o uso no terminal.

Output Estilizado

Interface de terminal com ASCII art, cores e logs formatados ([+], [*], [-]).

🛠️ Como Executar a Ferramenta

Pré-requisitos

Rust: Você precisa ter o ambiente de desenvolvimento Rust instalado (via rustup).

Dependências de Compilação (Linux): Em distribuições como o Kali/Debian, pode ser necessário instalar ferramentas de desenvolvimento:

sudo apt install pkg-config libssl-dev


1. Clonar e Acessar o Repositório

git clone [https://github.com/NoHup-lgtm/FerrisFetch.git](https://github.com/NoHup-lgtm/FerrisFetch.git)
cd FerrisFetch


2. Execução via Linha de Comando (CLI)

Use cargo run -- seguido da URL completa. O separador -- é essencial para passar a URL ao seu programa.

Exemplo de Uso:

cargo run -- [https://www.reidoscoins.com.br/](https://www.reidoscoins.com.br/)


Exemplo de Output:

# Exemplo de saída no terminal:
 ╔═╗╔═╗╦═╗╔═╗╦═╗╦ ╦╔═╗╦ ╦
 ║ ║╠═╝╠╦╝╠═╣╠╦╝║║║╠═╣╚╦╝
 ╚═╝╩  ╩╚═╩ ╩╩╚═╚╩╝╩ ╩ ╩ 
    >> FERRIS FETCH <<
...
[+] Attempting to crawl URL: [https://www.reidoscoins.com.br/](https://www.reidoscoins.com.br/)
[+] Found 48 unique assets/links for download.

[*] Downloading (1/48) [Main HTML] -> [https://www.reidoscoins.com.br/](https://www.reidoscoins.com.br/)
[+] Saved Main HTML to: downloads/index.html
[-] Waiting 2 seconds (base delay)...
[*] Downloading (2/48) [CSS] -> [https://www.reidoscoins.com.br/style.css](https://www.reidoscoins.com.br/style.css)
[+] Status SUCESSO: 200 OK 
[+] Saved to: downloads/style.css
...


⚙️ Configuração Interna

Você pode ajustar o comportamento do Crawler editando as constantes no arquivo src/main.rs:

Constante

Padrão

Descrição

MAX_RETRIES

5

Número máximo de vezes que o programa tentará baixar um ativo após uma falha.

INITIAL_DELAY_SECONDS

2

Tempo de espera (em segundos) entre o download de cada ativo/link. Ajuste este valor para evitar bloqueio.

RETRY_DELAY_MULTIPLIER

2

Fator pelo qual o atraso é multiplicado em caso de erro 429 (ex: 2s, 4s, 8s, 16s...).

📄 Licença

Este projeto está licenciado sob a Licença MIT.

Autor: NoHup-lgtm
