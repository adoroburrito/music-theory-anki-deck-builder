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

fn new_id(prev_id: usize) -> usize {
    prev_id + 1
}

fn get_relative_note(note: &str, interval: &str) -> &'static str {
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

fn main() -> Result<(), Error> {
    let mut curr_id: usize = 1;

    println!("Building note intervals deck...");
    // note relations
    let mut note_intervals_deck = Deck::new(
        new_id(curr_id),
        "Note intervals Deck",
        "Deck for note intervals",
    );

    for note in NOTES {
        for interval in INTERVALS {
            note_intervals_deck.add_note(Note::new(
                basic_model(),
                vec![
                    &format!("{} of {}", interval, note),
                    get_relative_note(note, interval),
                ],
            )?);
        }
    }

    note_intervals_deck.write_to_file("note_relations.apkg")?;

    Ok(())
}
