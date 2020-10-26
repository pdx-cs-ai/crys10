use std::{collections, env, fs, io, sync::atomic};

use atomic::AtomicUsize;
use atomic::Ordering::SeqCst as OSC;
use collections::HashSet as Set;
use io::BufRead;

use rayon::prelude::*;

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
    let nop: usize = env::args().nth(1).unwrap().parse().unwrap();

    let dict = io::BufReader::new(
        fs::File::open("/usr/local/share/dict/freq.txt").unwrap(),
    );
    let words = collect_words(dict, nop..=nop+1);

    let ntrips = AtomicUsize::new(0);
    words
        .par_iter()
        .enumerate()
        .filter(|(_, w)| w.len == nop)
        .for_each(|(i, w1)| {
            for (j, w2) in words[i + 1..].iter().enumerate() {
                if w2.len != nop {
                    continue;
                }
                let letters: Set<char> =
                    w1.letters.union(&w2.letters).cloned().collect();
                if letters.len() > nop {
                    continue;
                }
                for (k, w3) in words.iter().enumerate() {
                    if i == k || j + i + 1 == k {
                        continue;
                    }
                    let nletters = letters.union(&w3.letters).count();
                    if nletters <= nop {
                        let old_ntrips = ntrips.fetch_add(1, OSC);
                        if old_ntrips % 1000000 == 0 {
                            println!(
                                "{} {} {} {}",
                                old_ntrips,
                                w1.word,
                                w2.word,
                                w3.word
                            );
                        }
                    }
                }
            }
        });

    println!("{}", ntrips.load(OSC));
}
