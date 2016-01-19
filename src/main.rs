use std::env::*;
use std::iter::*;
use std::process::exit;

fn main() {
    let args : Vec<_> = std::env::args().collect();

    if args.len() <= 1 {
        println!("Usage: anagrams word");
        std::process::exit(0);
    }
    
    let word : String = args[1].clone();
    println!("Permutations of {:?}", word);

    let mut count = 0;

    let letters = word.into();

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


// Implementation of memory-constant lazy cloning.

pub struct PermutationGenerator<T>{
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
    pub fn new(source: Vec<T>) -> Self {
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
        // Edge case: are we done yet?
        if self.done {
            return None;
        }

        // Edge case: is there anything to do?
        let len = self.source.len();

        if len == 0 {
            return None;
        }

        // Edge case: are we even started?
        if !self.started {
            self.started = true;
            return Some(&self.latest);
        }


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


/// Test that we have the right permutations for "abc"
#[cfg(test)]
mod test {
    use super::*;
    use std::hash::Hash;
    use std::collections::HashSet;

    fn lazy_to_set<T>(letters: Vec<T>) -> HashSet<Vec<T>> where T: Clone + Eq + Hash + Ord {
        let reference = letters.clone().sort();
        let mut iterator = PermutationGenerator::new(letters);
        let mut permutations = HashSet::new();
        loop {
            if let Some(anagram) = iterator.next() {
                // Make sure that our anagrams use the right letters, with the right multiplicity.
                let sorted = anagram.clone().sort();
                assert_eq!(sorted, reference);

                // Make sure that our anagrams are distinct. Won't work if the same letter appears
                // more than once in `letters`.
                let fresh = permutations.insert(anagram.clone());
                assert!(fresh);
            } else {
                break;
            }
        }
        permutations
    }
    
    #[test]
    fn test_abcde() {
        let permutations = lazy_to_set("abcde".into());
        assert_eq!(permutations.len(), 120);
    }

    #[test]
    fn test_empty() {
        let permutations = lazy_to_set("".into());
        assert_eq!(permutations.len(), 0);
    }
}
