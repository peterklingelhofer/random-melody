use rand::seq::SliceRandom;
use rand::thread_rng;
use rodio::{source::SineWave, OutputStream, Source};
use std::time::Duration;

struct MusicalScale {
    notes: Vec<f32>,
}

impl MusicalScale {
    // Function to create a new scale
    fn new(notes: Vec<f32>) -> Self {
        MusicalScale { notes }
    }

    // Function to get a random note from the scale
    fn get_random_note(&self) -> f32 {
        let mut rng = thread_rng();
        *self.notes.choose(&mut rng).unwrap()
    }
}

fn main() {
    let major_blues_scale = MusicalScale::new(vec![
        261.63, // C
        293.66, // D
        311.13, // D#
        329.63, // E
        392.00, // G
        440.00, // A
    ]);

    let scale = &major_blues_scale;

    let melody_length = 8; // Length of the melody
    let mut melody = Vec::new();

    for _ in 0..melody_length {
        melody.push(scale.get_random_note());
    }

    // Audio playback
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    for &note in &melody {
        let source = SineWave::new(note).take_duration(Duration::from_secs_f32(rand::random::<f32>()));
        stream_handle.play_raw(source.convert_samples()).unwrap();

        // This is a simple pause between notes
        std::thread::sleep(Duration::from_secs(1));
    }
}

