use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;


fn main() {
    // open file wordlist.txt
    let lines = read_lines("wordlist.txt").unwrap();

    // read file line by line, add to set
    let mut set = std::collections::HashSet::new();
    for line in lines {
        set.insert(line.unwrap());
    }

    // for each word in set, check if the reverse is in set too, but if reverse is the same as the word, skip
    // save the word and the reverse to file semordnilap.txt
    let mut semordnilap = File::create("semordnilap.txt").unwrap();
    for word in set.iter() {
        let reverse: String = word.chars().rev().collect();
        if word != &reverse && set.contains(&reverse) {
            writeln!(semordnilap, "{} <> {}", word, reverse).unwrap();
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