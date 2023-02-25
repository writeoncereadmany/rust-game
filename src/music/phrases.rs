pub fn tuplet(note_length: f32, notes: Vec<f32>) -> Vec<(f32, f32, f32)> {
    notes.iter().enumerate().map(|(index, &note)| { (index as f32 * note_length, note_length, note) }).collect()
}