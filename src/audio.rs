use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{HeapRb, Consumer}; 
use anyhow::{Result, anyhow};
use std::sync::Arc;

pub struct AudioInput {
    cons: Consumer<i16, Arc<HeapRb<i16>>>,
    _stream: cpal::Stream,
}

unsafe impl Send for AudioInput {}
unsafe impl Sync for AudioInput {}

impl AudioInput {
    pub fn new(sample_rate: u32) -> Result<Self> {
        let host = cpal::default_host();
        let device = host.default_input_device()
            .ok_or_else(|| anyhow!("Perangkat audio tidak ditemukan"))?;

        let config = cpal::StreamConfig {
            channels: 1,
            sample_rate: cpal::SampleRate(sample_rate),
            buffer_size: cpal::BufferSize::Default,
        };

        let rb = HeapRb::<i16>::new(8192);
        let (mut prod, cons) = rb.split();

        let stream = device.build_input_stream(
            &config,
            move |data: &[i16], _| {
                for &sample in data {
                    let _ = prod.push(sample);
                }
            },
            |err| eprintln!("Audio error: {}", err),
            None
        )?;

        stream.play()?;

        Ok(Self {
            cons,
            _stream: stream,
        })
    }

    pub fn read(&mut self) -> Vec<i16> {
        let mut buffer = Vec::new();
        while let Some(sample) = self.cons.pop() {
            buffer.push(sample);
        }
        buffer
    }
}
