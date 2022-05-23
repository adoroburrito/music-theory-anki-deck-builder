use genanki_rs::{basic_model, Deck, Error, Note};
use std::{
    cmp::{Ord, Ordering},
    collections::HashMap,
};

const NOTES: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

const INTERVALS: [&str; 6] = [
    "third",
    "fifth",
    "seventh",
    "ninth",
    "eleventh",
    "thirteenth",
];

enum ToneMeasure {
    Tone,
    SemiTone,
}

const MAJOR_HARMONIC_INTERVALS: [&ToneMeasure; 7] = [
    &ToneMeasure::Tone,
    &ToneMeasure::Tone,
    &ToneMeasure::SemiTone,
    &ToneMeasure::Tone,
    &ToneMeasure::Tone,
    &ToneMeasure::Tone,
    &ToneMeasure::SemiTone,
];

fn get_relative_interval(note: &str, interval: &str) -> &'static str {
    let semitones_in_intervals: HashMap<&str, usize> = HashMap::from([
        ("third", 4),
        ("fifth", 7),
        ("seventh", 10),
        ("ninth", 13),
        ("eleventh", 16),
        ("thirteenth", 19),
    ]);

    let semitones_to_add = match semitones_in_intervals.get(interval) {
        Some(semitones_to_add) => semitones_to_add,
        None => panic!("Unknown interval: {}", interval),
    };

    // find the index of root
    let index = NOTES.binary_search(&note).unwrap();

    let max = NOTES.len() - 1;
    let target = index + semitones_to_add;
    // target is relative index
    match target.cmp(&max) {
        Ordering::Less | Ordering::Equal => NOTES[target],
        Ordering::Greater => {
            let wrapped_index = target % NOTES.len();
            NOTES[wrapped_index]
        }
    }
}

fn get_relative_note(note: &str, semitones_to_add: usize) -> &'static str {
    // find the index of root
    let index = NOTES.binary_search(&note).unwrap();

    let max = NOTES.len() - 1;
    let target = index + semitones_to_add;
    // target is relative index
    match target.cmp(&max) {
        Ordering::Less | Ordering::Equal => NOTES[target],
        Ordering::Greater => {
            let wrapped_index = target % NOTES.len();
            NOTES[wrapped_index]
        }
    }
}

fn get_major_harmonic_scale(note: &'static str) -> [&'static str; 8] {
    let mut semitones_to_add: usize = 0;
    let mut scale = ["?"; 8];

    scale[0] = note;
    for (i, interval) in MAJOR_HARMONIC_INTERVALS.iter().enumerate() {
        match *interval {
            ToneMeasure::SemiTone => semitones_to_add += 1,
            ToneMeasure::Tone => semitones_to_add += 2,
        }

        let actual_index = i + 1;
        scale[actual_index] = get_relative_note(note, semitones_to_add);
    }

    scale
}

fn main() -> Result<(), Error> {
    let mut curr_id: usize = 1;

    println!("Building note intervals deck...");
    let mut note_intervals_deck =
        Deck::new(curr_id, "Note intervals Deck", "Deck for note intervals");

    for note in NOTES {
        for interval in INTERVALS {
            note_intervals_deck.add_note(Note::new(
                basic_model(),
                vec![
                    &format!("{} of {}", interval, note),
                    get_relative_interval(note, interval),
                ],
            )?);
        }
    }

    note_intervals_deck.write_to_file("note_relations.apkg")?;
    curr_id += 1;
    println!("Done.");

    println!("Building major harmonic scales deck...");
    let mut major_harmonic_scales_deck = Deck::new(
        curr_id,
        "Major harmonic scales deck",
        "Deck for major harmonic scales",
    );
    for note in NOTES {
        major_harmonic_scales_deck.add_note(Note::new(
            basic_model(),
            vec![
                &format!("Major harmonic scale of {}", note),
                &get_major_harmonic_scale(note).join(" "),
            ],
        )?);
    }

    major_harmonic_scales_deck.write_to_file("major_harmonic_scales.apkg")?;
    curr_id += 1;
    println!("Done.");

    Ok(())
}
