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


/// Implementation of memory-constant, linear-time generation of permutations.
///
/// The algorithm is based on the remark that a permutation of [a_0, a_1, ... a_n]
/// is
/// 1. One substitution between a_0 and any of a_0, a_1, ... a_n
/// 2. One substitution between a_1 and any of a_1, a_2, ..., a_n
/// 3. One substitution between a_2 and any of a_2, ..., a_n
/// ...
/// n+1. One substitution between a_n and a_n, i.e. an empty substitution.
///
/// In other words, to generate a permutation, it is sufficient to generate
/// 1. One number in [0, n]
/// 2. One number in [1, n]
/// 3. One number in [2, n]
/// 4. ..
/// 5. One number in [n, n]
///
/// or, equivalently,
///
/// 1. One number in [0, 0]
/// 2. One number in [0, 1]
/// ...
/// n+1 One number in [0, n]
///
/// or, equivalently, an array of numbers limited by [0, 1, ... n]
///
/// We can easily generate the sequence of these arrays, in place, by starting
/// with [0, 0, ..., 0] and, at each call to `next()` increasing the value.
/// We start with the first index in the array, if incremeneting it would cause
/// an overflow, we reset it to 0 and rather proceed with the next index, etc.
/// If we have an overflow with all numbers, we have finished our enumeration.
///
///
/// # Example
/// ```
/// let mut iterator = PermutationGenerator::new("grunt-last".into());
/// loop {
///        if let Some(anagram) = iterator.next() {
///            println!("{:?}", String::from_utf8_lossy(anagram));
///        } else {
///            break;
///        }
///    }
/// ```
pub struct PermutationGenerator<T>{
    /// The original vector.
    source: Vec<T>,

    /// Decomposition of the current permutation as an array of transpositions.
    /// By definition, `indices[i] == j` means that the current permutation
    /// transposes `i` and `len - j + i` in `source`.
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

    /// A buffer used to write the latest permutation.
    /// Overwritten each time we call `next()`.
    ///
    /// Invariant: Always a permutation of `source`.
    /// Invariant: Always the same length as `source`.
    latest: Vec<T>,

    /// `true` once we have started the iteration. Used to handle the edge-case
    /// of the first permutation (the identity permutation).
    started: bool,

    /// `true` once we have seen all permutations.
    done: bool,
}

impl<T> PermutationGenerator<T> where T: Clone {
    /// Create a new permutation generator.
    ///
    /// Once `new()` has returned, this generator will not cause memory allocations.
    pub fn new(source: Vec<T>) -> Self {
        let len = source.len();

        let mut latest = Vec::with_capacity(len);
        for i in 0 .. len {
            latest.push(source[i].clone());
        }
        assert_eq!(len, latest.len());
        
        let mut indices = Vec::with_capacity(len);
        indices.resize(len, 0);
        assert_eq!(len, indices.len());

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
    /// Generate the next permutation.
    ///
    /// # Performance
    ///
    /// Calls to `next()` do not allocate memory.
    ///
    /// Each call to `next()` runs in linear time and typically quite
    /// fast (at most three walks through an array of length
    /// `source.len()`).
    ///
    ///
    /// # Result
    ///
    /// The resulting vector is pre-allocated during the call to
    /// `new()` and mutated at each call to `next()`. In other words,
    /// you cannot hold to it.
    ///
    /// # Example
    /// ```
    /// let mut iterator = PermutationGenerator::new("grunt-last".into());
    /// loop {
    ///        if let Some(anagram) = iterator.next() {
    ///            println!("{:?}", String::from_utf8_lossy(anagram));
    ///        } else {
    ///            break;
    ///        }
    ///    }
    /// ```
    fn next(&'a mut self) -> Option<&'a Vec<T>> {
        // Edge case: are we done yet?
        if self.done {
            return None;
        }

        // Edge case: are we even started?
        if !self.started {
            self.started = true;
            return Some(&self.latest);
        }

        // Edge case: is there anything to do?
        let len = self.source.len();

        if len == 0 {
            return None;
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


#[cfg(test)]
mod test {
    use super::*;
    use std::hash::Hash;
    use std::collections::HashSet;

    fn fact(n: u64) -> u64 {
        let mut result = 1;
        for i in 2 .. n + 1 {
            result *= i;
        }
        result
    }

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

    fn test_with<T>(letters: Vec<T>) where T: Clone + Eq + Hash + Ord {
        let fact = fact(letters.len() as u64);
        let permutations = lazy_to_set(letters);
        assert_eq!(permutations.len() as u64, fact);
    }

    #[test]
    fn test_ranges() {
        let source = "01234567";
        for i in 0 .. source.len() {
            let (slice, _) = source.split_at(i);
            test_with(slice.into());
        }
    }
}
