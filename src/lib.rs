#![feature(test)]
extern crate test;

use anyhow::Result;
use std::{fs::OpenOptions, io::Read};

pub struct KeyWord<'a> {
    pub word: &'a str,
    pub files: Vec<&'a str>,
}

#[derive(Debug)]
pub struct Found<'a> {
    pub word: &'a str,
    pub infiles: Vec<&'a str>,
}

pub trait Search {
    fn search_word(&self) -> Result<Found>;
}

impl<'a> Search for KeyWord<'a> {
    fn search_word(&self) -> Result<Found> {
        let mut found: Found = Found {
            word: "",
            infiles: Vec::new(),
        };

        for file in &self.files {
            let mut matching_chars = 0;
            let mut cursor = 0;

            let word = self.word.as_bytes();
            let mut open_file = OpenOptions::new().read(true).open(file)?;
            let mut buf: String = String::new();
            open_file.read_to_string(&mut buf)?;

            for letter in buf.as_bytes() {
                if cursor == word.len() {
                    cursor = 0;
                }

                let current = word[cursor];

                if current == *letter {
                    matching_chars += 1;
                    cursor += 1;
                } else {
                    matching_chars = 0;
                    cursor = 0;
                }

                if matching_chars == word.len() {
                    found.word = self.word;
                    found.infiles.push(file);
                    break;
                }
            }
        }

        Ok(found)
    }
}

#[cfg(test)]
mod tests {
    use std::process::Termination;

    use test::Bencher;

    use crate::{KeyWord, Search};

    #[test]
    fn match_words() {
        let key = KeyWord {
            word: "cool",
            files: vec!["test_words.txt"],
        };
        let search = key.search_word();

        assert_eq!(search.as_ref().unwrap().infiles.len(), 1);
    }

    #[bench]
    fn bench_search(b: &mut Bencher) -> impl Termination  {
        let key = KeyWord {
            word: "ZZZ",
            files: vec!["test_words.txt"],
        };
        b.iter(|| key.search_word())
    }
}
