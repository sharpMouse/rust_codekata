use crate::common::{WORDS_DATA, WORDS};

use bit_vec::BitVec;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

//////////////////////////////////////////////////////////////////////////////

type HashType = u64;

fn calc_hash(word: &str) -> HashType {
    let mut hasher = DefaultHasher::new();
    word.hash(&mut hasher);
    hasher.finish()
}
    
fn get_hashes(words: &str) -> Vec<HashType> {
    words
        .lines()
        .map(|word| calc_hash(word))
        .collect()
}

struct BloomFilter {
    bools: BitVec,
}

impl BloomFilter {
    fn new(bits_size: usize, hashes: &[HashType]) -> BloomFilter {
        let mut bools = BitVec::from_elem(bits_size, false);
        for hash in hashes {
            let offset = *hash as usize % bits_size;
            bools.set(offset, true);
        }
        BloomFilter{ bools }
    }

    #[cfg(test)]
    fn contains(&self, word: &str) -> bool {
        let offset = calc_hash(word) as usize % self.bools.len();
        self.bools.get(offset).unwrap()
    }
}

//////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
const RIGHT_WORDS: [&str; 5] = ["basic", "forum", "tree", "will", "power"];
#[cfg(test)]
const WRONG_WORDS: [&str; 5] = ["baaac", "fooom", "teee", "wiil", "pooor"];

fn bits_count(bits: &BitVec) -> usize {
    let mut count = 1;
    for block in bits.blocks() {
        count += block.count_ones();
    }
    count as usize
}

#[test]
fn test_filter() {
    let word_hashes = get_hashes(&WORDS_DATA);
    
    let filter = BloomFilter::new(16*1024, &word_hashes);
    assert!(filter.contains(WRONG_WORDS[0])); // small size causes lots of collisions
    assert!(filter.contains(WRONG_WORDS[4])); // the same

    let filter = BloomFilter::new(4024*1024, &word_hashes); // large enough to avoid collisions
    for word in RIGHT_WORDS {
        assert!(WORDS.contains(word), "{}", word);
        assert!(filter.contains(word), "{}", word);
    }
    for word in WRONG_WORDS {
        assert!(!WORDS.contains(word), "{}", word);
        assert!(!filter.contains(word), "{}", word);
    }
}

pub fn print_stats() {
    let now = std::time::Instant::now();
    println!("Kata5: Bloom Filter");
    println!("Words: {}", WORDS.len());
    println!("Longest word: {}", WORDS.iter().max_by_key(|w| w.len()).unwrap());
    
    let word_hashes = get_hashes(&WORDS_DATA);
    let sizes = [128, 256, 512, 1024, 2048, 4096, 10240];
    for size in sizes {
        let filter = BloomFilter::new(size*1024, &word_hashes);
        let len = filter.bools.len();
        let bits = bits_count(&filter.bools);
        let colls = (WORDS.len() - bits) * 100 / bits;
        println!("Bits: total {:8}, set {:6}, collisions {}%", len, bits, colls);
    }
    println!("Elapsed: {:.2?}", now.elapsed());
}
