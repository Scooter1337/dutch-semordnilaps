use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::vec::Vec;

fn main() {

    let mut ongekeurd_set = std::collections::HashSet::new();
    let mut gekeurd_set = std::collections::HashSet::new();

    let ongekeurd = read_lines("basiswoorden-ongekeurd.txt").unwrap();
    for line in ongekeurd {
        ongekeurd_set.insert(line.unwrap().to_lowercase());
    }

    let gekeurd = read_lines("basiswoorden-gekeurd.txt").unwrap();
    for line in gekeurd {
        gekeurd_set.insert(line.unwrap().to_lowercase());
    }

    semordnilap("nemordnilap-gekeurd.txt", &gekeurd_set);
    semordnilap("nemordnilap-ongekeurd.txt", &ongekeurd_set);
    palindromes("palindromen-gekeurd.txt", &gekeurd_set);
    palindromes( "palindromen-ongekeurd.txt", &ongekeurd_set);
    anagrams("anagrammen-gekeurd.txt", &gekeurd_set);
    anagrams("anagrammen-ongekeurd.txt", &ongekeurd_set);
}

fn semordnilap(output: &str, set: &std::collections::HashSet<String>) {
    let mut found = std::collections::HashSet::new();

    // for each word in set, check if the reverse is in set too, but if reverse is the same as the word, skip
    // save the word and the reverse to file semordnilap.txt
    let mut semordnilap = File::create(output).unwrap();
    set.iter().for_each(|word| {
        if word.len() < 2 { return; }

        // skip words consisting of 1 character
        if word.chars().all(|c| c == word.chars().next().unwrap()) { return; }

        let reverse: String = word.chars().rev().collect();
        if word != &reverse && !found.contains(&reverse) && set.contains(&reverse) {
            found.insert(word);
            writeln!(semordnilap, "{}, {}", word, reverse).unwrap();
        }
    })
}

fn palindromes(output: &str, set: &std::collections::HashSet<String>) {
    // for each word in set, check if the reverse is the same as the word
    let mut palindromes = File::create(output).unwrap();

    set.iter().for_each(|word| {
        // skip words with less than 2 characters
        if word.len() < 2 { return; }

        // skip words consisting of 1 character
        if word.chars().all(|c| c == word.chars().next().unwrap()) { return; }

        let reverse: String = word.chars().rev().collect();
        if word == &reverse {
            writeln!(palindromes, "{}", word).unwrap();
        }
    })
}

fn anagrams(output: &str, set: &std::collections::HashSet<String>) {
    // open file wordlist.txt


    // read file line by line, add to map (word, sorted word)
    let mut map: HashMap<String, Vec<String>> = std::collections::HashMap::new();
    for word in set.iter() {
        // skip words with less than 2 characters
        if word.len() < 2 { continue; }

        // skip words consisting of 1 character
        if word.chars().all(|c| c == word.chars().next().unwrap()) { continue; }

        let sorted = sort_word(word);
        let wordlist = map.get_mut(&sorted);

        match wordlist {
            Some(words) => {
                words.push(word.to_owned());
            },
            None => {
                map.insert(sorted, vec![word.to_owned()]);
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

/// The output is wrapped in a Result to allow matching on errors. \
/// Returns an Iterator to the Reader of the lines of the file. \
/// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Sorts the characters of a word alphabetically \
/// foobar -> abfoor
fn sort_word(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}