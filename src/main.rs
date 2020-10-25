use std::{fs, io, collections};
use io::BufRead;
use collections::HashSet as Set;

struct Word {
    word: String,
    letters: Set<char>,
    len: usize,
}

fn collect_words<T>(dict: io::BufReader<T>, ns: impl Iterator<Item=usize>) -> Vec<Word>
    where T: io::Read
{
    let sizes: Set<usize> = ns.collect();
    println!("{:?}", sizes);
    dict
        .lines()
        .map(|line| line.unwrap())
        .filter(|word| sizes.contains(&word.len()))
        .map(|word| {
            let letters = word.chars().collect();
            let len = word.len();
            let word = word.to_string();
            Word { word, letters, len }
        })
        .collect()
}

fn main() {
    let dict = io::BufReader::new(
        fs::File::open("/usr/local/share/dict/freq.txt").unwrap(),
    );
    let words = collect_words(dict, 10..=11);
    println!("{}", words.len());
}
