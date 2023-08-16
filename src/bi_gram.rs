use rand::{rngs::ThreadRng, Rng};
use std::{collections::HashMap, fs, fs::File, hash::Hash, io::Read};

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

#[derive(Debug)]
struct NextWordOption {
    next: String,
    probability: f64,
}

impl PartialEq for NextWordOption {
    fn eq(&self, other: &Self) -> bool {
        return self.next == other.next && self.probability == other.probability;
    }
}
impl Eq for NextWordOption {}
impl PartialOrd for NextWordOption {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.probability > other.probability {
            return Some(std::cmp::Ordering::Less);
        } else if self.probability < other.probability {
            return Some(std::cmp::Ordering::Greater);
        } else {
            return Some(std::cmp::Ordering::Equal);
        }
    }
}
impl Ord for NextWordOption {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.partial_cmp(other).unwrap();
    }
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

fn get_bigrams_freqs(contents: &str) -> FreqsAndOccurences {
    let mut freqs = HashMap::<BiGram, u32>::new();
    let mut occurences = HashMap::<&str, u32>::new();
    let mut it = contents.split(" ");
    let mut prev = it.next().unwrap();
    for item in it {
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

fn read_files_into_string(paths: &[String]) -> String {
    let mut res = String::new();
    for path in paths {
        let mut f = File::open(path).unwrap();
        f.read_to_string(&mut res).unwrap();
    }
    return res;
}

fn create_bi_gram(freqs_and_occurrences: FreqsAndOccurences) -> BiGramModel {
    let bigrams = freqs_and_occurrences.freqs;
    let occurences = freqs_and_occurrences.occurences;

    let mut res_map = HashMap::<String, BiGramOptions>::new();
    for (key, value) in bigrams {
        let occurence_count = occurences.get(key.first).unwrap();
        let probability = value as f64 / *occurence_count as f64;

        let option = NextWordOption {
            next: String::from(key.second),
            probability,
        };
        match res_map.get_mut(key.first) {
            Some(val) => {
                val.opts.push(option);
                val.opts.sort(); // TODO: is this necessary
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
    pub fn new(path: &str) -> BiGramModel {
        let contents = fs::read_to_string(path).unwrap();
        let freqs_and_occurences = get_bigrams_freqs(contents.as_str());
        return create_bi_gram(freqs_and_occurences);
    }

    pub fn new_multiple(paths: &[String]) -> BiGramModel {
        let contents = read_files_into_string(paths);
        let freqs_and_occurrences = get_bigrams_freqs(contents.as_str());
        return create_bi_gram(freqs_and_occurrences);
    }

    pub fn get_next(&mut self, first: &str, rng: &mut ThreadRng) -> Option<&str> {
        let val = self.data.get(first)?;
        let num: f64 = rng.gen_range(0.0..val.sum);

        let mut it = val.opts.iter();
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
