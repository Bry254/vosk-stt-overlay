use vosk::{Model, Recognizer};
pub struct VoskReconizer {
    model: Model,
    reconizer: Recognizer,
    pub acumulated: String,
}
impl VoskReconizer {
    pub fn new(model_path: &str) -> Self {
        let modelo = Model::new(model_path).unwrap();
        let reconizer = Recognizer::new(&modelo, 16000.0).unwrap();
        Self {
            model: modelo,
            reconizer: reconizer,
            acumulated: String::new(),
        }
    }
    pub fn process_audio(&mut self, samples: &[i16]) -> String {
        self.reconizer
            .accept_waveform(samples)
            .expect("Error al procesar audio");
        let result = self.reconizer.partial_result();
        return result.partial.to_string();
    }
    pub fn process_audio_with_limit(&mut self, samples: &[i16], limit: usize) -> String {
        let parcial = self.process_audio(samples);
        if parcial.len() > limit {
            self.acumulated = parcial.clone();
            self.reset();
            return parcial;
        }
        return format!("{} {}", self.acumulated, parcial.clone());
    }
    pub fn reset(&mut self) {
        self.reconizer =
            Recognizer::new(&self.model, 16000.0).expect("No se pudo reiniciar el recognizer");
    }
}
