use std::{collections, fs, io};

use collections::HashSet as Set;
use io::BufRead;

struct Word {
    #[allow(unused)]
    word: String,
    letters: Set<char>,
    len: usize,
}

fn collect_words<T>(
    dict: io::BufReader<T>,
    ns: impl Iterator<Item = usize>,
) -> Vec<Word>
where
    T: io::Read,
{
    let sizes: Set<usize> = ns.collect();
    dict.lines()
        .map(|line| line.unwrap())
        .filter(|word| sizes.contains(&word.len()))
        .map(|word| {
            let letters = word.chars().collect();
            let len = word.len();
            Word { word, letters, len }
        })
        .collect()
}

fn main() {
    let dict = io::BufReader::new(
        fs::File::open("/usr/local/share/dict/freq.txt").unwrap(),
    );
    let words = collect_words(dict, 10..=11);

    let mut ntrips = 0;
    for (i, w1) in words.iter().enumerate() {
        if w1.len != 10 {
            continue;
        }
        for (j, w2) in words[i + 1..].iter().enumerate() {
            if w2.len != 10 {
                continue;
            }
            let letters: Set<char> =
                w1.letters.union(&w2.letters).cloned().collect();
            if letters.len() > 10 {
                continue;
            }
            for (k, w3) in words.iter().enumerate() {
                if i == k || j == k {
                    continue;
                }
                let nletters = letters.union(&w3.letters).count();
                if nletters <= 10 {
                    ntrips += 1;
                    if ntrips % 1000000 == 0 {
                        println!(
                            "{} {} {} {}",
                            ntrips, w1.word, w2.word, w3.word
                        );
                    }
                }
            }
        }
    }

    println!("{}", ntrips);
}
