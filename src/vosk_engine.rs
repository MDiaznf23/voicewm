use vosk::{Model, Recognizer, DecodingState};
use anyhow::{Result, anyhow};
use tokio::sync::mpsc;
use log::{info, warn, error};
use crate::audio::AudioInput;

pub struct VoskStream {
    model: Model,
    sample_rate: f32,
}

impl VoskStream {
    pub fn new(model_path: &str, sample_rate: f32) -> Result<Self> {
        // Memuat model Vosk
        let model = Model::new(model_path)
            .ok_or_else(|| anyhow!("Gagal memuat model Vosk dari path: {}", model_path))?;
        
        Ok(Self { 
            model, 
            sample_rate 
        })
    }

    pub async fn start(self, tx: mpsc::UnboundedSender<String>) -> Result<()> {
        let mut recognizer = Recognizer::new(&self.model, self.sample_rate)
            .ok_or_else(|| anyhow!("Gagal inisialisasi recognizer Vosk"))?;

        let mut audio_input = AudioInput::new(self.sample_rate as u32)?;
        
        info!("Vosk Engine Aktif. Mendengarkan perintah...");

        loop {
            let buffer = audio_input.read(); 
            
            if !buffer.is_empty() {
                match recognizer.accept_waveform(&buffer) {
                    Ok(DecodingState::Finalized) => {
                        let result = recognizer.result().single().unwrap().text;
                        if !result.is_empty() {
                            let _ = tx.send(result.to_string());
                        }
                    }
                    Ok(DecodingState::Running) => {
                    }
                    Ok(DecodingState::Failed) => {
                        warn!("Vosk decoding failed for this chunk");
                    }
                    Err(e) => {
                        error!("Vosk error: {:?}", e);
                    }
                }
            }

            tokio::task::yield_now().await;
            tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;
        }
    }
}
