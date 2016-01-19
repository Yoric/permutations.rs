use std::env::*;
use std::iter::*;

fn main() {
    let args : Vec<_> = std::env::args().collect();
    let word : String = args[1].clone();
    println!("Permutations of {:?}", word);

    let letters = word.into();
/*
    for anagram in anagrams(letters) {
        println!("{:?}", String::from_utf8(anagram).unwrap());
    }
*/

    let mut count = 0;
    let mut iterator = PermutationGenerator::new(letters);
    loop {
        if let Some(anagram) = iterator.next() {
            count += 1;
            println!("{:?}", String::from_utf8_lossy(anagram));
        } else {
            break;
        }
    }

    println!("Generated {} permutations", count);
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

struct PermutationGenerator<T>{
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
    started: bool,
    done: bool,
}

impl<T> PermutationGenerator<T> where T: Clone {
    fn new(source: Vec<T>) -> Self {
        let len = source.len();

        let mut latest = Vec::with_capacity(len);
        for i in 0 .. len {
            latest.push(source[i].clone());
        }

        let mut indices = Vec::with_capacity(len);
        indices.resize(len, 0);

        PermutationGenerator {
            started: false,
            done: false,
            source: source,
            indices: indices,
            latest: latest,
        }
    }
}

impl<'a, T> PermutationGenerator<T> where T: Clone {
    fn next(&'a mut self) -> Option<&'a Vec<T>> {
        if self.done {
            return None;
        }

        if !self.started {
            self.started = true;
            return Some(&self.latest);
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
        
        // Prepare result
        for i in 0 .. len {
            self.latest[i] = self.source[i].clone();
        }

        // Pick the item in `i`.
        for i in 0 .. len - 1 {
            let delta = self.indices[len - 1 - i]; // In 0 .. len - i - 1
            self.latest.swap(i, i + delta);
        }
        

        if done {
            self.done = true;
            return None;
        }


        Some(&self.latest)
    }
}


#[test]
fn it_works() {
}
