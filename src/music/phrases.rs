use crate::audio::tempo::Phrase;

pub fn tuplet(note_length: f32, notes: Vec<f32>) -> Phrase {
    notes.iter().enumerate().map(|(index, &note)| { (index as f32 * note_length, note_length, note) }).collect()
}