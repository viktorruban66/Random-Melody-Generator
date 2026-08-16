// melody_generator.rs — Rust версия

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::Duration;

lazy_static::lazy_static! {
    static ref NOTES: HashMap<String, f64> = {
        let mut m = HashMap::new();
        m.insert("C4".to_string(), 261.63);
        m.insert("C#4".to_string(), 277.18);
        m.insert("D4".to_string(), 293.66);
        m.insert("D#4".to_string(), 311.13);
        m.insert("E4".to_string(), 329.63);
        m.insert("F4".to_string(), 349.23);
        m.insert("F#4".to_string(), 369.99);
        m.insert("G4".to_string(), 392.00);
        m.insert("G#4".to_string(), 415.30);
        m.insert("A4".to_string(), 440.00);
        m.insert("A#4".to_string(), 466.16);
        m.insert("B4".to_string(), 493.88);
        m.insert("C5".to_string(), 523.25);
        m.insert("C#5".to_string(), 554.37);
        m.insert("D5".to_string(), 587.33);
        m.insert("D#5".to_string(), 622.25);
        m.insert("E5".to_string(), 659.25);
        m.insert("F5".to_string(), 698.46);
        m.insert("F#5".to_string(), 739.99);
        m.insert("G5".to_string(), 783.99);
        m.insert("G#5".to_string(), 830.61);
        m.insert("A5".to_string(), 880.00);
        m.insert("A#5".to_string(), 932.33);
        m.insert("B5".to_string(), 987.77);
        m.insert("C6".to_string(), 1046.50);
        m
    };
    static ref DURATIONS: Vec<String> = vec![
        "1/16".to_string(), "1/8".to_string(), "1/4".to_string(),
        "1/2".to_string(), "1".to_string()
    ];
    static ref DURATION_VALUES: HashMap<String, f64> = {
        let mut m = HashMap::new();
        m.insert("1/16".to_string(), 0.0625);
        m.insert("1/8".to_string(), 0.125);
        m.insert("1/4".to_string(), 0.25);
        m.insert("1/2".to_string(), 0.5);
        m.insert("1".to_string(), 1.0);
        m
    };
}

struct MelodyGenerator {
    num_notes: usize,
    tempo: u32,
    notes_pool: Vec<String>,
    melody: Vec<(String, String)>,
}

impl MelodyGenerator {
    fn new(num_notes: usize, tempo: u32, start: &str, end: &str) -> Self {
        let pool: Vec<String> = NOTES.keys()
            .filter(|&n| n >= start && n <= end)
            .cloned()
            .collect();
        let pool = if pool.is_empty() { NOTES.keys().cloned().collect() } else { pool };
        MelodyGenerator {
            num_notes,
            tempo,
            notes_pool: pool,
            melody: Vec::new(),
        }
    }

    fn generate(&mut self) {
        let mut rng = thread_rng();
        self.melody.clear();
        for _ in 0..self.num_notes {
            let note = self.notes_pool.choose(&mut rng).unwrap().clone();
            let dur = DURATIONS.choose(&mut rng).unwrap().clone();
            self.melody.push((note, dur));
        }
    }

    fn play(&self) {
        for (note, dur) in &self.melody {
            let freq = NOTES[note];
            let duration_ms = DURATION_VALUES[dur] * (60.0 / self.tempo as f64) * 1000.0;
            let _ = Command::new("beep")
                .args(&["-f", &(freq as i32).to_string(), "-l", &(duration_ms as i32).to_string()])
                .status();
            thread::sleep(Duration::from_millis((duration_ms * 0.1) as u64));
        }
    }

    fn save(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        writeln!(file, "Note\tFreq (Hz)\tDuration")?;
        for (note, dur) in &self.melody {
            writeln!(file, "{}\t{:.2}\t{}", note, NOTES[note], dur)?;
        }
        println!("💾 Сохранено: {}", filename);
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut num_notes = 8;
    let mut tempo = 120;
    let mut range = "C4-C6".to_string();
    let mut output = "melody.txt".to_string();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--notes" | "-n" => { num_notes = args[i+1].parse().unwrap_or(8); i += 2; }
            "--tempo" | "-t" => { tempo = args[i+1].parse().unwrap_or(120); i += 2; }
            "--range" | "-r" => { range = args[i+1].clone(); i += 2; }
            "--output" | "-o" => { output = args[i+1].clone(); i += 2; }
            _ => { i += 1; }
        }
    }
    let parts: Vec<&str> = range.split('-').collect();
    let start = if parts.len() > 0 { parts[0].trim() } else { "C4" };
    let end = if parts.len() > 1 { parts[1].trim() } else { "C6" };

    let mut gen = MelodyGenerator::new(num_notes, tempo, start, end);
    gen.generate();
    println!("\x1b[36m🎵 Random Melody Generator (Rust)\x1b[0m");
    println!("Параметры: {} нот, диапазон {}-{}, темп {} BPM\n", num_notes, start, end, tempo);
    println!("Сгенерированная мелодия:");
    for (note, dur) in &gen.melody {
        println!("  {} ({:.2} Гц) {}", note, NOTES[note], dur);
    }
    println!("\nВоспроизведение...");
    gen.play();
    let _ = gen.save(&output);
}
