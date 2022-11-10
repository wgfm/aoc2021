use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Read},
    ops::{Deref, DerefMut},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

struct Polymer {
    pairs: Pairs,
    rules: HashMap<(char, char), char>,
    ends: (char, char),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Pairs(HashMap<(char, char), u64>);

impl Pairs {
    fn new() -> Self {
        Self(HashMap::new())
    }
}

impl From<&str> for Pairs {
    fn from(state: &str) -> Self {
        let mut pairs = HashMap::new();
        for i in 0..(state.len() - 1) {
            let key = (
                state.chars().nth(i).unwrap(),
                state.chars().nth(i + 1).unwrap(),
            );
            if let Some(count) = pairs.get_mut(&key) {
                *count += 1;
            } else {
                pairs.insert(key, 1);
            }
        }

        Pairs(pairs)
    }
}

impl Deref for Pairs {
    type Target = HashMap<(char, char), u64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Pairs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Polymer {
    fn new(state: &str, rules: HashMap<(char, char), char>) -> Self {
        let mut chars = state.chars();
        let ends = (chars.next().unwrap(), chars.last().unwrap());

        Self {
            pairs: state.into(),
            rules,
            ends,
        }
    }

    fn from_reader<R>(input: R) -> Result<Self>
    where
        R: Read,
    {
        let mut r = BufReader::new(input);
        let mut state = String::new();
        r.read_line(&mut state)?;

        let state = state.trim_end().to_owned();

        r.read_line(&mut String::new())?;

        let mut rules = HashMap::new();
        for line in r.lines() {
            let line = line?;
            let mut from = '?';
            let mut to = '?';
            let mut ins = '?';
            for (i, ch) in line.char_indices() {
                match i {
                    0 => from = ch,
                    1 => to = ch,
                    6 => ins = ch,
                    _ => {}
                }
            }
            rules.insert((from, to), ins);
        }

        Ok(Polymer::new(&state, rules))
    }

    fn evolve(&mut self) {
        let mut new_pairs = Pairs::new();
        for ((l, r), count) in self.pairs.iter() {
            if let Some(m) = self.rules.get(&(*l, *r)) {
                if let Some(lm_count) = new_pairs.get_mut(&(*l, *m)) {
                    *lm_count += count;
                } else {
                    new_pairs.insert((*l, *m), *count);
                }

                if let Some(mr_count) = new_pairs.get_mut(&(*m, *r)) {
                    *mr_count += count;
                } else {
                    new_pairs.insert((*m, *r), *count);
                }
            }
        }

        self.pairs = new_pairs;
    }

    fn count_elements(&self) -> HashMap<char, u64> {
        let mut m = HashMap::new();
        m.insert(self.ends.0, 1);
        m.insert(self.ends.1, 1);

        let mut counts = self.pairs.iter().fold(m, |mut acc, ((l, r), pair_count)| {
            if let Some(count) = acc.get_mut(r) {
                *count += pair_count;
            } else {
                acc.insert(*r, *pair_count);
            }

            if let Some(count) = acc.get_mut(l) {
                *count += pair_count;
            } else {
                acc.insert(*l, *pair_count);
            }

            acc
        });

        counts.iter_mut().for_each(|(_, ct)| *ct /= 2);

        counts
    }
}

fn first() -> Result<String> {
    let f = File::open("input/14.in")?;
    let mut poly = Polymer::from_reader(f)?;

    for _ in 0..10 {
        poly.evolve();
    }

    let counts = poly.count_elements();
    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();

    Ok(format!("{}", max - min))
}

fn second() -> Result<String> {
    let f = File::open("input/14.in")?;
    let mut poly = Polymer::from_reader(f)?;

    for _ in 0..40 {
        poly.evolve();
    }

    let counts = poly.count_elements();
    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();

    Ok(format!("{}", max - min))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_sample() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        let mut poly = Polymer::from_reader(input.as_bytes()).expect("failed to parse polymer");
        poly.evolve();
        assert_eq!(poly.pairs, Pairs::from("NCNBCHB"));

        poly.evolve();
        assert_eq!(poly.pairs, Pairs::from("NBCCNBBBCBHCB"));

        poly.evolve();
        assert_eq!(poly.pairs, Pairs::from("NBBBCNCCNBBNBNBBCHBHHBCHB"));

        poly.evolve();
        assert_eq!(
            poly.pairs,
            Pairs::from("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"),
        );
    }

    #[test]
    fn count_elems() {
        let poly = Polymer::new("ABBCCCDDDD", HashMap::new());
        let counts = poly.count_elements();
        assert_eq!(counts.get(&'A'), Some(&1));
        assert_eq!(counts.get(&'B'), Some(&2));
        assert_eq!(counts.get(&'C'), Some(&3));
        assert_eq!(counts.get(&'D'), Some(&4));
    }

    // #[test]
    fn test_first() {
        assert_eq!("asdf".to_owned(), second().unwrap());
    }
}
