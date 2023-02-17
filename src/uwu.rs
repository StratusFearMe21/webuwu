#![cfg_attr(all(feature = "bench", test), feature(test))]

use std::fmt::{Error, Write};
use std::str::SplitWhitespace;
use std::sync::Mutex;

use ahash::RandomState;
use futures_signals::signal::Mutable;
use linkify::{LinkFinder, LinkKind};

use crate::constants::{ACTIONS, MIXED_FACES};

macro_rules! new_seeder {
    ($word:expr,$seeder:expr) => {
        <rand_xoshiro::Xoshiro256Plus as rand::SeedableRng>::seed_from_u64($seeder.hash_one($word))
    };
}

macro_rules! random_float {
    ($seeder:expr) => {
        rand::Rng::gen_range($seeder, 0.0..1.0)
    };
}

macro_rules! random_int {
    ($seeder:expr, $range:expr) => {
        rand::Rng::gen_range($seeder, $range)
    };
}

#[derive(Default, Debug)]
pub struct Token {
    stutter: bool,
    word: String,
    face: Option<usize>,
    action: Option<usize>,
}

#[derive(Debug)]
pub struct UwUify {
    random: Mutex<RandomState>,
    pub words: Mutable<f64>,
    pub faces: Mutable<f64>,
    pub actions: Mutable<f64>,
    pub stutters: Mutable<f64>,
    linkify: LinkFinder,
}

impl Default for UwUify {
    fn default() -> Self {
        Self {
            random: Mutex::new(RandomState::with_seeds(69, 420, 96, 84)),
            words: Mutable::new(1.0),
            faces: Mutable::new(0.05),
            actions: Mutable::new(0.125),
            stutters: Mutable::new(0.225),
            linkify: {
                let mut linkify = LinkFinder::new();
                linkify.kinds(&[LinkKind::Email, LinkKind::Url]);
                linkify.url_must_have_scheme(false);
                linkify
            },
        }
    }
}

#[derive(Debug)]
pub struct UwUIter<'a>(SplitWhitespace<'a>, &'a UwUify);

impl<'a> Iterator for UwUIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(word) = self.0.next() else {
            return None;
        };

        let mut seeder = new_seeder!(word, &self.1.random.lock().unwrap());
        let random_value = random_float!(&mut seeder);

        let words = self.1.words.get();
        let faces = self.1.faces.get();
        let actions = self.1.actions.get();
        let stutters = self.1.stutters.get();

        let mut token = Token::default();

        if random_value <= faces {
            token.face = Some(random_int!(&mut seeder, 0..MIXED_FACES.len()));
        }

        if random_value <= actions {
            token.action = Some(random_int!(&mut seeder, 0..ACTIONS.len()));
        }

        token.stutter = random_value <= stutters;

        if self.1.linkify.links(word).count() > 0 || random_value > words {
            token.word = word.to_owned();
        } else {
            let mut chars = word.chars();

            while let Some(w) = chars.next() {
                match w {
                    'L' | 'R' => token.word.write_char('W').unwrap(),
                    'l' | 'r' => token.word.write_char('w').unwrap(),
                    c @ ('N' | 'n') => {
                        token.word.write_char(c).unwrap();
                        if let Some(w) = chars.next() {
                            match w {
                                'L' | 'R' => token.word.write_char('W').unwrap(),
                                'l' | 'r' => token.word.write_char('w').unwrap(),
                                c @ ('A' | 'E' | 'I' | 'O' | 'U' | 'a' | 'e' | 'i' | 'o' | 'u') => {
                                    token.word.write_char('y').unwrap();
                                    token.word.write_char(c).unwrap();
                                }
                                c => token.word.write_char(c).unwrap(),
                            }
                        }
                    }
                    c => token.word.write_char(c).unwrap(),
                };
            }
        }
        Some(token)
    }
}

impl UwUify {
    pub fn new_seed(&self) {
        let mut seed_one = [0u8; 8];
        let mut seed_two = [0u8; 8];
        let mut seed_three = [0u8; 8];
        let mut seed_four = [0u8; 8];
        getrandom::getrandom(&mut seed_one).unwrap();
        getrandom::getrandom(&mut seed_two).unwrap();
        getrandom::getrandom(&mut seed_three).unwrap();
        getrandom::getrandom(&mut seed_four).unwrap();
        *self.random.lock().unwrap() = RandomState::with_seeds(
            u64::from_ne_bytes(seed_one),
            u64::from_ne_bytes(seed_two),
            u64::from_ne_bytes(seed_three),
            u64::from_ne_bytes(seed_four),
        );
    }

    pub fn uwuify_iter<'a>(&'a self, text: &'a str) -> UwUIter<'a> {
        UwUIter(text.split_whitespace(), &self)
    }

    pub fn uwuify_sentence<T: Write>(&self, text: &str, out: &mut T) -> Result<(), Error> {
        self.uwuify_iter(text).try_for_each(|word| {
            if let Some(face) = word.face {
                out.write_str(MIXED_FACES[face])?;
            }

            if let Some(action) = word.action {
                out.write_str(ACTIONS[action])?;
            }

            if word.stutter {
                out.write_fmt(format_args!("{}-", word.word.chars().next().unwrap_or('W')))?;
            }

            out.write_str(&word.word)?;
            out.write_char(' ')
        })
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "bench")]
    extern crate test;

    #[cfg(feature = "bench")]
    #[bench]
    fn uwu_bench(b: &mut test::Bencher) {
        let uwuify = super::UwUify::new(
            Some(include_str!("test.txt")),
            None,
            None,
            false,
            true,
            false,
            None,
            None,
            None,
            None,
            false,
        );
        b.iter(|| uwuify.uwuify());
    }
}
