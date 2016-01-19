use std::env::*;
use std::iter::*;
use std::marker::*;

fn main() {
    let args : Vec<_> = std::env::args().collect();
    let word : String = args[1].clone();
    println!("Anagrams of {:?}", word);

    let letters = word.into();
    for anagram in anagrams(letters) {
        println!("{:?}", String::from_utf8(anagram).unwrap());
    }
}


// Implementation of eager cloning.

fn cloneMinus<T>(original: &Vec<T>, index: usize) -> Vec<T> where T: Clone {
    let mut dest = Vec::with_capacity(original.len() - 1);
    for i in 0 .. original.len() {
        if i == index {
            continue;
        }
        dest.push(original[i].clone());
    }
    dest
}

fn anagrams<T>(letters: Vec<T>) -> Vec<Vec<T>> where T: Clone {
    if letters.len() <= 1 {
        return vec![letters];
    }
    let mut result = Vec::new();
    for i in 0 .. letters.len() {
        let letter = letters[i].clone();
        let subset = cloneMinus(&letters, i);
        let anagrams = anagrams(subset);
        for anagram in anagrams {
            for j in 0 .. anagram.len() {
                let mut copy = anagram.clone();
                copy.insert(j, letter.clone());
                result.push(copy.clone());
            }
        }
    }
    result
}

// Implementation of memory-constant lazy cloning.

struct AnagramIterator<'a, T> where T: 'a {
    /// The original vector.
    source: Vec<T>,

    /// Decomposition of the current index.
    ///
    /// Invariant: `indices.len()` is the same as `source.len()`.
    /// Invariant:
    ///  indices[0] ranges over [0, 0]
    ///  indices[1] ranges over [0, 1]
    ///   ...
    ///  indices[i] ranges over [0, i]
    ///   ...
    ///  indices[indices.len() - 1] ranges over [0, indices.len() - 1]
    indices: Vec<usize>,
    latest: Vec<T>,
    done: bool,
    phantom: PhantomData<&'a T>
}

impl<'a, T> AnagramIterator<'a, T> where T: Clone {
    fn new(source: Vec<T>) -> Self {
        let len = source.len();

        let mut latest = Vec::with_capacity(len);
        for i in 0 .. len {
            latest[i] = source[i].clone()
        }

        let mut indices = Vec::with_capacity(len);
        indices.resize(len, 0);

        AnagramIterator {
            done: false,
            source: source,
            indices: indices,
            latest: latest,
            phantom: PhantomData,
        }
    }

    fn get(&'a self) -> &'a Vec<T> {
        &self.latest
    }
}

impl<'a, T> Iterator for AnagramIterator<'a, T> where T: Clone {
    type Item = ();
    fn next(&mut self) -> Option<()> {
        if self.done {
            return None;
        }

        let len = self.source.len();
        
        // Increase indices
        let mut done = true;
        for i in 0 .. len {
            let index = self.indices[i] + 1;
            if index >= i + 1 {
                self.indices[i] = 0;
            } else {
                self.indices[i] = index;
                done = false;
                break;
            }
        }

        if done {
            self.done = true;
            return None;
        }

        // Prepare result
        for i in 0 .. len {
            self.latest[i] = self.source[i].clone();
        }

        // Pick the item in `i`.
        for i in 0 .. len - 1 {
            let delta = self.indices[len - 1 - i]; // In 0 .. len - i - 1
            self.latest.swap(i, i + delta);
        }

        Some(())
    }
}

#[test]
fn it_works() {
}
