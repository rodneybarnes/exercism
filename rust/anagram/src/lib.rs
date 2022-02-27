use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|a| is_anagrammatical_match(word, **a))
        .map(|r| *r)
        .collect::<HashSet<&'a str>>()
}

fn is_anagrammatical_match(base_word: &str, possible_match: &str) -> bool {
    if base_word.len() != possible_match.len() {
        return false;
    }

    let lowercase_base_word = base_word.to_lowercase();
    let lowercase_possible_match = possible_match.to_lowercase();

    if lowercase_base_word == lowercase_possible_match {
        return false;
    }

    // check if the total number of times each letter appears in the base word matches the total number of times
    // it appears in the possible match

    let mut base_word_letters = HashMap::new();
    for letter in lowercase_base_word.graphemes(true) {
        let counter = base_word_letters.entry(letter).or_insert(0);
        *counter += 1;
    }

    let mut possible_match_letters = HashMap::new();
    for letter in lowercase_possible_match.graphemes(true) {
        let counter = possible_match_letters.entry(letter).or_insert(0);
        *counter += 1;
    }

    for (letter, occurrences) in base_word_letters {
        match possible_match_letters.get(letter) {
            Some(occ) => {
                if occ != &occurrences {
                    return false;
                }
            },
            None => { return false; }
        }
    }

    true
}
