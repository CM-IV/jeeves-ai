use anyhow::Result;
use clap::Parser;
use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};
use owo_colors::OwoColorize;
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

/// Jeeves, your personal AI helper in the terminal
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The question to ask Jeeves
    #[arg(short, long)]
    question: String,

}

#[tokio::main]
async fn main() -> Result<()> {

    let args = Args::parse();

    // For custom values:
    let ollama = Ollama::new("Your Ollama Server Address".to_string(), 11434);

    let model = "Your AI Model Name".to_string();

    let prompt = args.question;

    let str = "\nJeeves is thinking...\n".to_string();

    println!("{}", str.purple());

    let mut stream = ollama.generate_stream(GenerationRequest::new(model, prompt)).await.unwrap();

    let mut stdout = tokio::io::stdout();

    while let Some(res) = stream.next().await {
        let res = res.unwrap();
        stdout.write(res.response.as_bytes()).await.unwrap();
        stdout.flush().await.unwrap();
    }

    Ok(())
}
