#![cfg_attr(all(feature = "bench", test), feature(test))]

use std::io::{Error, Write};
use std::str::from_utf8_unchecked;
use std::sync::Mutex;

use ahash::RandomState;
use futures_signals::signal::Mutable;
use linkify::{LinkFinder, LinkKind};

use crate::constants::{
    ACTIONS, ACTIONS_SIZE, ASCII_FACES, ASCII_FACES_SIZE, MIXED_FACES, MIXED_FACES_SIZE,
    UNICODE_FACES, UNICODE_FACES_SIZE,
};

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

macro_rules! write {
    ($out:expr, $bytes:expr) => {
        $out.write_all($bytes)
    };
}

#[derive(Debug)]
pub struct UwUify {
    ascii_only: bool,
    unicode_only: bool,
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
            ascii_only: true,
            unicode_only: false,
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

    pub fn uwuify_sentence<T: Write>(&self, text: &str, out: &mut T) -> Result<(), Error> {
        let words = self.words.get();
        let faces = self.faces.get();
        let actions = self.actions.get();
        let stutters = self.stutters.get();

        text.lines().try_for_each(|line| {
            line.split_whitespace()
                .map(|word_str| word_str.as_bytes())
                .try_for_each(|word| {
                    let mut seeder = new_seeder!(word, &self.random.lock().unwrap());
                    let random_value = random_float!(&mut seeder);

                    if random_value <= faces {
                        if self.ascii_only {
                            write!(
                                out,
                                ASCII_FACES[random_int!(&mut seeder, 0..ASCII_FACES_SIZE)]
                            )?;
                        } else if self.unicode_only {
                            write!(
                                out,
                                UNICODE_FACES[random_int!(&mut seeder, 0..UNICODE_FACES_SIZE)]
                            )?;
                        } else {
                            write!(
                                out,
                                MIXED_FACES[random_int!(&mut seeder, 0..MIXED_FACES_SIZE)]
                            )?;
                        }
                    }
                    if random_value <= actions {
                        write!(out, ACTIONS[random_int!(&mut seeder, 0..ACTIONS_SIZE)])?;
                    }
                    if random_value <= stutters {
                        match word[0] {
                            b'L' | b'R' if random_value < words => write!(out, b"W"),
                            b'l' | b'r' if random_value < words => write!(out, b"w"),
                            byte => write!(out, &[byte]),
                        }?;
                        write!(out, b"-")?;
                    }

                    if self
                        .linkify
                        .links(unsafe { from_utf8_unchecked(word) })
                        .count()
                        > 0
                        || random_value > words
                    {
                        write!(out, word)?;
                    } else {
                        (0..word.len()).try_for_each(|index| match word[index] {
                            b'L' | b'R' => write!(out, b"W"),
                            b'l' | b'r' => write!(out, b"w"),
                            b'A' | b'E' | b'I' | b'O' | b'U' | b'a' | b'e' | b'i' | b'o' | b'u' => {
                                match word.get(index - 1).unwrap_or(&word[0]) {
                                    b'N' | b'n' => write!(out, &[b'y', word[index]]),
                                    _ => write!(out, &[word[index]]),
                                }
                            }
                            byte => write!(out, &[byte]),
                        })?;
                    }
                    write!(out, b" ")
                })?;
            write!(out, b"\n")
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
