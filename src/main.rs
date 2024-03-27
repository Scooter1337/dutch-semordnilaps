use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::vec::Vec;

fn main() {
    semordnilap("basiswoorden-gekeurd.txt", "nemordnilap-gekeurd.txt");
    semordnilap("basiswoorden-ongekeurd.txt", "nemordnilap-ongekeurd.txt");
    palindromes("basiswoorden-gekeurd.txt", "palindromen-gekeurd.txt");
    palindromes("basiswoorden-ongekeurd.txt", "palindromen-ongekeurd.txt");
    anagrams("basiswoorden-gekeurd.txt", "anagrammen-gekeurd.txt");
    anagrams("basiswoorden-ongekeurd.txt", "anagrammen-ongekeurd.txt");
}

fn semordnilap(input: &str, output: &str) {
    // open file wordlist.txt
    let lines = read_lines(input).unwrap();

    // read file line by line, add to set
    let mut set = std::collections::HashSet::new();
    for line in lines {
        set.insert(line.unwrap().to_lowercase());
    }

    let mut found = std::collections::HashSet::new();

    // for each word in set, check if the reverse is in set too, but if reverse is the same as the word, skip
    // save the word and the reverse to file semordnilap.txt
    let mut semordnilap = File::create(output).unwrap();
    for word in set.iter() {
        if word.len() < 2 { continue; }

        // skip words consisting of 1 character
        if word.chars().all(|c| c == word.chars().next().unwrap()) { continue; }

        let reverse: String = word.chars().rev().collect();
        if word != &reverse && !found.contains(&reverse) && set.contains(&reverse) {
            found.insert(word);
            writeln!(semordnilap, "{}, {}", word, reverse).unwrap();
        }
    }
}

fn palindromes(input: &str, output: &str) {
    // open file wordlist.txt
    let lines = read_lines(input).unwrap();

    // read file line by line, add to set
    let mut set = std::collections::HashSet::new();
    for line in lines {
        set.insert(line.unwrap().to_lowercase());
    }

    // for each word in set, check if the reverse is the same as the word
    let mut palindromes = File::create(output).unwrap();
    for word in set.iter() {

        // skip words with less than 2 characters
        if word.len() < 2 { continue; }

        // skip words consisting of 1 character
        if word.chars().all(|c| c == word.chars().next().unwrap()) { continue; }

        let reverse: String = word.chars().rev().collect();
        if word == &reverse {
            writeln!(palindromes, "{}", word).unwrap();
        }
    }
}

fn anagrams(input: &str, output: &str) {
    // open file wordlist.txt
    let lines = read_lines(input).unwrap();

    // read file line by line, add to map (word, sorted word)
    let mut map: HashMap<String, Vec<String>> = std::collections::HashMap::new();
    for line in lines {
        let word = line.unwrap().to_lowercase();
        let sorted = sort_word(&word);
        let wordlist = map.get_mut(&sorted);

        match wordlist {
            Some(words) => {
                words.push(word);
            },
            None => {
                map.insert(sorted, vec![word]);
            }
        }
    }

    // for each sorted word in map, if the length of the list of words is greater than 1, save to file anagrammen.txt
    let mut anagrams = File::create(output).unwrap();

    for (_, words) in map.iter() {
        if words.len() > 1 {
            let words_joined = words.join(", ");
            writeln!(anagrams, "{}", words_joined).unwrap();
        }

    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn sort_word(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}