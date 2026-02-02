mod audio;
mod command; 
mod config;
mod vosk_engine; 

use anyhow::Result;
use clap::Parser;
use log::{error, info};
use tokio::sync::mpsc;
use tokio::signal;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: Option<String>,

    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    let config_path = args.config.unwrap_or_else(|| config::Config::default_path());
    info!("Loading config from: {}", config_path);
    
    let config = config::Config::load(&config_path)?;
    info!("Config loaded successfully. Model Path: {}", config.general.model_path);
    info!("Total commands registered: {}", config.commands.len());

    let matcher = command::CommandMatcher::new(config.commands.clone());

    let (tx, mut rx) = mpsc::unbounded_channel();

    let vosk = vosk_engine::VoskStream::new(
        &config.general.model_path,
        config.audio.sample_rate as f32,
    )?;

    info!("Entering Voice Mode (Vosk Engine). Listening for commands...");

    tokio::spawn(async move {
        if let Err(e) = vosk.start(tx).await {
            error!("Vosk engine error: {}", e);
        }
    });

    loop {
        tokio::select! {
            Some(text) = rx.recv() => {
                info!("Detected speech: '{}'", text);
                
                let matcher_ref = matcher.clone();
                tokio::spawn(async move {
                    if let Err(e) = matcher_ref.match_and_execute(&text).await {
                        error!("Command execution error: {}", e);
                    }
                });
            }

            _ = signal::ctrl_c() => {
                info!("Shutdown signal received. Exiting Voice Mode...");
                break;
            }
        }
    }

    Ok(())
}
