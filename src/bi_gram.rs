use rand::{rngs::ThreadRng, Rng};
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{Error, Read};

struct BiGram<'a> {
    first: &'a str,
    second: &'a str,
}

impl<'a> PartialEq for BiGram<'a> {
    fn eq(&self, other: &Self) -> bool {
        return self.first == other.first && self.second == other.second;
    }
}
impl<'a> Eq for BiGram<'a> {}

impl<'a> Hash for BiGram<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.first.hash(state);
        self.second.hash(state);
    }
}

struct NextWordOption {
    next: String,
    probability: f64,
}

struct BiGramOptions {
    sum: f64,
    opts: Vec<NextWordOption>,
}

pub struct BiGramModel {
    data: HashMap<String, BiGramOptions>,
}

struct FreqsAndOccurences<'a> {
    freqs: HashMap<BiGram<'a>, u32>,
    occurences: HashMap<&'a str, u32>,
}

fn get_bi_gram_freqs(contents: &str) -> FreqsAndOccurences {
    let mut freqs = HashMap::<BiGram, u32>::new();
    let mut occurences = HashMap::<&str, u32>::new();
    let mut it = contents.split(char::is_whitespace);
    let opt = it.next();
    let mut prev;
    match opt {
        Some(val) => {
            prev = val;
        }
        None => {
            return FreqsAndOccurences { freqs, occurences };
        }
    }
    while prev.is_empty() {
        prev = it.next().unwrap();
    }
    for item in it {
        if item.is_empty() {
            continue;
        }
        let bigram = BiGram {
            first: prev,
            second: item,
        };
        freqs
            .entry(bigram)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        match occurences.get_mut(prev) {
            Some(count) => *count += 1,
            None => {
                occurences.insert(prev, 1);
            }
        }
        prev = item;
    }
    return FreqsAndOccurences { freqs, occurences };
}

fn read_files_into_string(paths: &[String]) -> Result<String, Error> {
    let mut res = String::new();
    for path in paths {
        let mut f = File::open(path)?;
        f.read_to_string(&mut res)?;
    }
    return Ok(res);
}

fn create_bi_gram(freqs_and_occurrences: FreqsAndOccurences) -> BiGramModel {
    let freqs = freqs_and_occurrences.freqs;
    let occurences = freqs_and_occurrences.occurences;

    let mut res_map = HashMap::<String, BiGramOptions>::new();
    for (key, value) in freqs {
        // unwrap fine because everything in freqs has an occurrence count
        let occurence_count = occurences.get(key.first).unwrap();
        let probability = value as f64 / *occurence_count as f64;

        let option = NextWordOption {
            next: String::from(key.second),
            probability,
        };
        match res_map.get_mut(key.first) {
            Some(val) => {
                val.opts.push(option);
                val.sum += probability;
            }
            None => {
                res_map.insert(
                    String::from(key.first),
                    BiGramOptions {
                        sum: probability,
                        opts: vec![option],
                    },
                );
            }
        }
    }

    return BiGramModel { data: res_map };
}

impl BiGramModel {
    pub fn new(paths: &[String]) -> Result<BiGramModel, Error> {
        let contents = read_files_into_string(paths)?;
        let freqs_and_occurrences = get_bi_gram_freqs(contents.as_str());
        return Ok(create_bi_gram(freqs_and_occurrences));
    }

    pub fn get_next(&self, first: &str, rng: &mut ThreadRng) -> Option<&str> {
        let val = self.data.get(first)?;
        let num: f64 = rng.gen_range(0.0..val.sum);

        let mut it = val.opts.iter();
        // on insertion at least one item has to have been inserted into val.opts
        let mut item = it.next().unwrap();
        let mut sum = val.sum - item.probability;
        while num < sum {
            match it.next() {
                Some(got_val) => {
                    item = got_val;
                    sum -= item.probability;
                }
                None => {
                    break;
                }
            }
        }
        return Some(item.next.as_str());
    }
}
