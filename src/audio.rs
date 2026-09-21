use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc;
use std::thread;

use crate::voskr;

pub fn start_audio_thread(model: &String) -> mpsc::Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    let mut reconocedor = voskr::VoskReconizer::new(model);
    thread::spawn(move || {
        println!("Comenzando a esuchar audio");
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .expect("No se encontro una entrada");
        let config = cpal::StreamConfig {
            channels: 1,
            sample_rate: 16000,
            buffer_size: cpal::BufferSize::Default,
        };
        println!("Detectado {:?}", config);
        let stream = device
            .build_input_stream(
                config,
                move |data: &[i16], _| {
                    // println!("enviando samples");
                    let samples = data.to_vec();
                    let texto = reconocedor.process_audio_with_limit(&samples, 150);
                    tx.send(texto).unwrap();
                    // let texto = reconocedor.process_audio(&samples);
                    // if texto.len() > 250 {
                    //     reconocedor.acumulated = texto;
                    //     reconocedor.reset();
                    // }
                    // tx.send(format!("{} {}", reconocedor.acumulated, texto).to_string())
                    //     .unwrap();
                },
                move |err| {
                    eprintln!("Error de audio: {}", err);
                },
                None,
            )
            .unwrap();
        stream.play().unwrap();
        loop {
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
    return rx;
}
