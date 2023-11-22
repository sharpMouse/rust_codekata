#![cfg(test)]
use crate::common::WordsStorage;
use itertools::Itertools;
use std::collections::HashMap;

type KeyType = Vec<u8>;

// The sorted word is the same for all anagrams
fn make_key(word: &str) -> KeyType {
    word.bytes().sorted().collect()
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct WordKey {
    origin: &'static str,
    key: KeyType,
}

// Bind the word and it's key
fn make_keys(words: &WordsStorage) -> Vec<WordKey> {
    words
        .iter()
        .map(|word| WordKey { origin: word, key: make_key(word) })
        .collect()
}

fn find_anagrams_by_sort(words: &WordsStorage) -> (usize, usize, usize, usize) {
    let mut keys = make_keys(words);
    
    keys.sort_unstable_by(|a, b| a.key.cmp(&b.key)); // dedup requires sorting
    
    let anagrams: Vec<_> = keys
        .iter()
        .dedup_by_with_count(|a, b| a.key == b.key) // make pairs (count, value)
        .filter(|x| x.0 > 1) // choose anagrams
        .collect();

    let set_count = anagrams.len();
    let word_count = anagrams.iter().map(|x| x.0).sum();
    let themost_size = anagrams.iter().map(|x| x.0).max().unwrap();
    let longest_size = anagrams.iter().map(|x| x.1.key.len()).max().unwrap();
    
    (set_count, word_count, themost_size, longest_size)
}

fn find_anagrams_by_hash(words: &WordsStorage) -> (usize, usize, usize, usize) {
    let keys = make_keys(words);
    
    let mut buckets = HashMap::new();
    for key in &keys {
        let vc = &mut buckets.entry(&key.key).or_insert(Vec::with_capacity(13));
        vc.push(key.origin);
    }
    
    let anagrams: Vec<_> = buckets.values().filter(|x| x.len() > 1).collect();
        
    let set_count = anagrams.len();
    let word_count = anagrams.iter().map(|x| x.len()).sum();
    let themost_size = anagrams.iter().map(|x| x.len()).max().unwrap();
    let longest_size = anagrams.iter().map(|x| x[0].len()).max().unwrap();
    
    (set_count, word_count, themost_size, longest_size)
}

#[test]
fn anagrams() {
    for func in [find_anagrams_by_sort, find_anagrams_by_hash] {
        let time = std::time::Instant::now();
        let (set_count, word_count, themost, longest) = func(&crate::common::WORDS);
        println!("time: {:.2?}\n", time.elapsed());
        
        assert_eq!(set_count, 20683);
        assert_eq!(word_count, 48162);
        assert_eq!(themost, 13); // alerts+
        assert_eq!(longest, 19); // acoustoelectrically
    }
    //assert!(false); uncomment this to show timings
}
