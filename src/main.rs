use genanki_rs::{basic_model, Deck, Error, Note};
use std::{
    cmp::{Ord, Ordering},
    collections::HashMap,
};

const NOTES: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

const NON_ACCIDENTAL_NOTES: [&str; 7] = ["A", "B", "C", "D", "E", "F", "G"];

const INTERVALS: [&str; 6] = [
    "third",
    "fifth",
    "seventh",
    "ninth",
    "eleventh",
    "thirteenth",
];

const GREEK_MODES: [&str; 7] = [
    "ionian",
    "dorian",
    "phrygian",
    "lidian",
    "mixolydian",
    "aeolian",
    "locrian",
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

fn get_chords_for_major_note(note: &str) -> [&'static str; 7] {
    let chords_for_notes: HashMap<&str, [&str; 7]> = HashMap::from([
        ("A", ["A", "Bm", "C#m", "D", "E", "F#m", "G#/5b"]),
        ("B", ["B", "C#m", "D#m", "E", "F#", "G#m", "A#m/5b"]),
        ("C", ["C", "Dm", "Em", "F", "G", "Am", "B/5b"]),
        ("D", ["D", "Em", "F#m", "G", "A", "Bm", "C#/5b"]),
        ("E", ["E", "F#m", "G#m", "A", "B", "C#m", "D#/5b"]),
        ("F", ["F", "Gm", "Am", "Bb", "C", "Dm", "E/5b"]),
        ("G", ["G", "Am", "Bm", "C", "D", "Em", "F#/5b"]),
    ]);

    *chords_for_notes.get(note).unwrap()
}

fn get_greek_mode_scale(note: &str) -> [&'static str; 7] {
    let scale_for_mode: HashMap<&str, [&str; 7]> = HashMap::from([
        ("ionian", ["T", "2", "3", "4", "5", "6", "7"]),
        ("dorian", ["T", "2", "3b", "4", "5", "6", "7b"]),
        ("phrygian", ["T", "2b", "3b", "4", "5", "6b", "7b"]),
        ("lidian", ["T", "2", "3", "4#", "5", "6", "7"]),
        ("mixolydian", ["T", "2", "3", "4", "5", "6", "7b"]),
        ("aeolian", ["T", "2", "3b", "4", "5", "6b", "7b"]),
        ("locrian", ["T", "2b", "3b", "4", "5b", "6b", "7b"]),
    ]);

    *scale_for_mode.get(note).unwrap()
}

fn get_greek_mode_quirk(note: &str) -> &'static str {
    let quirk_for_mode: HashMap<&str, &str> = HashMap::from([
        ("ionian", "happy, energetic, popular"),
        ("dorian", "latino, swing, party, dance"),
        ("phrygian", "enigma, spanish, flamenco"),
        ("lidian", "unexpected, imponent"),
        ("mixolydian", "blues, country, regional"),
        ("aeolian", "sad, introspective, reflective, hope"),
        ("locrian", "unstable, conflict, tense"),
    ]);

    *quirk_for_mode.get(note).unwrap()
}

fn get_greek_mode_tonal_signature(note: &str) -> &'static str {
    let signature_for_mode: HashMap<&str, &str> = HashMap::from([
        ("ionian", "perfect 4th"),
        ("dorian", "major 6th"),
        ("phrygian", "minor 2nd"),
        ("lidian", "augmented 4th"),
        ("mixolydian", "minor 7th"),
        ("aeolian", "minor 6th"),
        ("locrian", "minor 2nd & diminished 5th"),
    ]);

    *signature_for_mode.get(note).unwrap()
}

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

    note_intervals_deck.write_to_file("decks/note_relations.apkg")?;
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

    for note in NON_ACCIDENTAL_NOTES {
        major_harmonic_scales_deck.add_note(Note::new(
            basic_model(),
            vec![
                &format!("Chords in {} major", note),
                &get_chords_for_major_note(note).join(" "),
            ],
        )?);
    }

    major_harmonic_scales_deck.write_to_file("decks/major_harmonic_scales.apkg")?;
    curr_id += 1;
    println!("Done.");
    println!("Building greek modes deck...");
    let mut greek_modes_deck = Deck::new(curr_id, "Greek modes deck", "Deck for greek modes");
    for mode in GREEK_MODES {
        greek_modes_deck.add_note(Note::new(
            basic_model(),
            vec![
                &format!("Scale for greek mode: {}", mode),
                &get_greek_mode_scale(mode).join(" "),
            ],
        )?);

        greek_modes_deck.add_note(Note::new(
            basic_model(),
            vec![
                &format!("Quirk for greek mode: {}", mode),
                &get_greek_mode_quirk(mode),
            ],
        )?);

        greek_modes_deck.add_note(Note::new(
            basic_model(),
            vec![
                &format!("Tonal signature for greek mode: {}", mode),
                &get_greek_mode_tonal_signature(mode),
            ],
        )?);
    }

    greek_modes_deck.write_to_file("decks/greek_modes.apkg")?;
    curr_id += 1;
    println!("Done.");

    Ok(())
}
