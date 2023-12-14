use anyhow::Result;
use clap::{Parser, Subcommand};
use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};
use owo_colors::OwoColorize;
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

/// Jeeves, your personal AI helper in the terminal
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,

}

#[derive(Subcommand)]
enum Commands {
    #[command(alias = "q")]
    /// Provide the model name and prompt
    Question {
        /// Required model name
        #[arg(short, long)]
        model_name: String,

        /// Required prompt to give to the model
        #[arg(short, long)]
        prompt: String
    },
    /// List available models to choose from
    List
    
}


#[tokio::main]
async fn main() -> Result<()> {

    let args = Args::parse();

    // For custom values:
    let ollama = Ollama::new("YOUR OLLAMA SERVER ADDRESS".to_string(), 11434);

    match args.command {
        Commands::Question { model_name, prompt } => {

            let str = "\nJeeves is thinking...\n".to_string();

            println!("{}", str.purple());
        
            let mut stream = ollama.generate_stream(GenerationRequest::new(model_name, prompt)).await.unwrap();
        
            let mut stdout = tokio::io::stdout();
        
            while let Some(res) = stream.next().await {
                let res = res.unwrap();
                stdout.write(res.response.as_bytes()).await.unwrap();
                stdout.flush().await.unwrap();
            }

        },
        Commands::List => {
            // Print each AIModel variant
            let models = ollama.list_local_models().await.unwrap();

            for variant in models.as_slice() {
                println!("{}", variant.name);
            }
        }
    }

    Ok(())
}
