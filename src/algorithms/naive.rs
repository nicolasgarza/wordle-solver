use crate::{Guess, Guesser, DICTIONARY, Correctness};
use std::collections::HashMap;

pub struct Naive {
    remaining: HashMap<&'static str, usize>,
}

impl Naive {
    pub fn new() -> Self {
        Naive {
            remaining: HashMap::from_iter(DICTIONARY.lines().map(|line| {
                let (word, count) = line
                    .split_once(' ')
                    .expect("every line is word + space + frequency");
                let count: usize = count.parse().expect("every count is a number");
                (word, count)
            })),
        }
    }
}
#[derive(Debug, Copy, Clone)]
struct Candidate {
    word: &'static str,
    goodness: f64,
}

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> String {
        if let Some(last) = history.last() {
            self.remaining.retain(|word, _| last.matches(word));
        }
        if history.is_empty() {
            return "tares".to_string();
        }

        let remaining_count: usize = self.remaining.iter().map(|(_, &c)| c).sum();

        let mut best: Option<Candidate> = None;
        for (&word, _) in &self.remaining {
            let mut sum = 0.0;
            for pattern in Correctness::patterns() {
                // considering a word where we _did_ guess `word` and got `pattern` as the 
                // correctness. Now, compute what _then_ is left
                let mut in_pattern_total = 0;
                for (candidate, count) in &self.remaining {
                
                    let g = Guess {
                        word: word.to_string(),
                        mask:pattern
                    };
                    if g.matches(candidate) {
                        in_pattern_total += count;
                    }
                }
                if in_pattern_total == 0 {
                    continue;
                }
                let p_of_this_pattern = in_pattern_total as f64 / remaining_count as f64;
                sum += p_of_this_pattern * p_of_this_pattern.log2();
            
            }
            let goodness = -sum;

            if let Some(c) = best {
                // is this one better?
                if goodness > c.goodness {
                    best = Some(Candidate {
                        word,
                        goodness,
                    });
                }
            } else {
                best = Some(Candidate {
                    word,
                    goodness,
                });
            }
        }
        best.unwrap().word.to_string()
    }
}
