//based on https://github.com/RustAudio/cpal/blob/1ac8f1549f41001acd0acef2be9214ab72e61d11/examples/beep.rs

use anyhow::{anyhow, Context};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

//we have to store the stream to keep the thread alive
pub struct Buzzer {
    pub(super) switch: Arc<AtomicBool>,
    _stream: cpal::Stream,
}

impl Buzzer {
    pub fn init() -> anyhow::Result<Self> {
        //default audio host and output device
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow!("Audio device initalisation error"))
            .context("Could not get default audio device")?;

        //get the first audio config
        let config = device
            .default_output_config()
            .context("Could not get default audio device")?;

        let switch = Arc::<AtomicBool>::default();

        //run audio stream
        //starts it's own background thread
        let stream = match config.sample_format() {
            SampleFormat::F32 => start::<f32>(&device, &config.into(), switch.clone()),
            SampleFormat::I16 => start::<i16>(&device, &config.into(), switch.clone()),
            SampleFormat::U16 => start::<u16>(&device, &config.into(), switch.clone()),
        }
        .context("Could not start audio stream")?;

        Ok(Buzzer {
            switch,
            _stream: stream,
        })
    }
}

fn start<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    volume_switch: Arc<AtomicBool>,
) -> anyhow::Result<cpal::Stream>
where
    T: cpal::Sample,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f32;
    let mut next_sample = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let data_fn = move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
        for frame in data.chunks_mut(channels) {
            let value: T = if volume_switch.load(Ordering::Relaxed) {
                Sample::from::<f32>(&next_sample())
            } else {
                Sample::from(&0.0)
            };

            for sample in frame.iter_mut() {
                *sample = value;
            }
        }
    };

    let stream = device
        .build_output_stream(config, data_fn, err_fn)
        .context("Could not build audio output stream")?;

    stream
        .play()
        .context("Could not start audio output stream")?;

    Ok(stream)
}
