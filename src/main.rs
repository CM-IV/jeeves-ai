use anyhow::{Result, anyhow};
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

enum AIModel {
    DeepSeekCoder,
    OpenHermes25Mistral,
    StableLmZephyr,
}

impl AIModel {
    fn as_str(&self) -> &'static str {
        match self {
            AIModel::DeepSeekCoder => "deepseek-coder:6.7b-instruct-q4_0",
            AIModel::OpenHermes25Mistral => "openhermes2.5-mistral",
            AIModel::StableLmZephyr => "stablelm-zephyr",
        }
    }
}

// Function to parse the AI model name
fn parse_ai_model(model: &str) -> Result<AIModel> {
    match model {
        "deepseek-coder:6.7b-instruct-q4_0" => Ok(AIModel::DeepSeekCoder),
        "openhermes2.5-mistral" => Ok(AIModel::OpenHermes25Mistral),
        "stablelm-zephyr" => Ok(AIModel::StableLmZephyr),
        _ => Err(anyhow!("Invalid AI model: {}", model)),
    }
}


#[tokio::main]
async fn main() -> Result<()> {

    let args = Args::parse();

    // For custom values:
    let ollama = Ollama::new("YOUR OLLAMA SERVER".to_string(), 11434);

    match args.command {
        Commands::Question { model_name, prompt } => {

            let ai_model = parse_ai_model(&model_name)?;

            let str = "\nJeeves is thinking...\n".to_string();

            println!("{}", str.purple());
        
            let mut stream = ollama.generate_stream(GenerationRequest::new(ai_model.as_str().to_owned(), prompt)).await.unwrap();
        
            let mut stdout = tokio::io::stdout();
        
            while let Some(res) = stream.next().await {
                let res = res.unwrap();
                stdout.write(res.response.as_bytes()).await.unwrap();
                stdout.flush().await.unwrap();
            }

        },
        Commands::List => {
            // Print each AIModel variant
            for variant in &[AIModel::DeepSeekCoder, AIModel::OpenHermes25Mistral, AIModel::StableLmZephyr] {
                println!("{}", variant.as_str());
            }
        }
    }

    Ok(())
}
