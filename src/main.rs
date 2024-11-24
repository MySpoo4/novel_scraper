use clap::Parser;
use novel_scraper::{
    models::{ExcludedWords, Novel},
    scraper::Runner,
};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    /// Path to the json config file
    #[arg(short, long, value_name = "NOVEL_PATH", required = true)]
    novel_path: PathBuf,
    /// Output path for epub
    #[arg(short, long, value_name = "OUTPUT_PATH", required = true)]
    output_path: PathBuf,
    /// Proxy url for http client
    #[arg(short, long, value_name = "PROXY_URL")]
    proxy_url: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let (novel, excluded_words) = get_fields(args.novel_path);

    let runner_result = Runner::new(novel, excluded_words, args.proxy_url.as_deref());
    match runner_result {
        Ok(mut runner) => runner
            .run(args.output_path)
            .await
            .map_or_else(|e| eprintln!("{}", e), |_| println!("Done!")),
        Err(e) => {
            eprintln!("Error creating Runner: {}", e);
        }
    }
}

fn get_fields(path: PathBuf) -> (Novel, ExcludedWords) {
    let file = fs::File::open(path).expect("Failed to read file");
    let json: Value = serde_json::from_reader(file).expect("Failed to parse file");

    let novel: Novel = serde_json::from_value(
        json.get("novel")
            .expect("novel field not found in json")
            .to_owned(),
    )
    .expect("failed to deserialize novel");

    let excluded = ExcludedWords {
        excluded_words: serde_json::from_value(
            json.get("excluded_words")
                .expect("exclude field not found in json")
                .to_owned(),
        )
        .expect("failed to deserialize excluded_words"),
    };

    (novel, excluded)
}
