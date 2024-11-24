# Novel Scraper
A simple program that scrapes novels from the internet.

To build binary:
```
cargo build --release
```
located here: `target/release/novel/novel_scraper`

Program expects
```
Options:
  -n, --novel-path <NOVEL_PATH>    Path to the json config file
  -o, --output-path <OUTPUT_PATH>  Output path for epub
  -p, --proxy-url <PROXY_URL>      Proxy url for http client (optional)
```
Example novel json file in repo.
