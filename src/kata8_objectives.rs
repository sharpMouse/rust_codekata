use crate::common::{WORDS_PLAIN, WordsStoragePlain};

// Readable variant //

fn words_with_len(len: usize) -> impl Iterator<Item = &'static str> {
    WORDS_PLAIN.iter().copied().filter(move |w| w.len() == len)
}

#[allow(dead_code)]
fn is_two_words(word: &str) -> usize {
    for len in 2..=4 {
        for subword1 in words_with_len(len) {
            if word.starts_with(subword1) {
                for subword2 in words_with_len(6 - len) {
                    if word.ends_with(subword2) {
                        return 1;
                    }
                }
            }
        }
    }
    return 0;
}

#[allow(dead_code)]
fn find_subwords_readable() -> usize {
    words_with_len(6).map(|w| is_two_words(w)).sum()
}

// Fast variant //

fn is_two_words_fast(word: &str) -> usize {
    for len in 2..=4 {
        let subword1 = String::from_iter(word.chars().take(len));
        if WORDS_PLAIN.binary_search(&subword1.as_str()).is_ok() {
            let subword2 = String::from_iter(word.chars().skip(len));
            if WORDS_PLAIN.binary_search(&subword2.as_str()).is_ok() {
                return 1;
            }
        }
    }
    return 0;
}

fn find_subwords_fast() -> usize {
    let n_words_iter = WORDS_PLAIN.iter().filter(|w| w.chars().count() == 6);
    n_words_iter.map(|w| is_two_words_fast(w)).sum()
}

// Extendible variant //

fn find_subwords_ext(words: &WordsStoragePlain, word_len: usize, min_len: usize) -> usize {
    let n_words_iter = words.iter().filter(|w| w.chars().count() == word_len);
    n_words_iter.map(|w| is_two_words_fast_ext(words, word_len, min_len, w)).sum()
}

fn is_two_words_fast_ext(words: &WordsStoragePlain, word_len: usize, min_len: usize, word: &str) -> usize {
    for len in min_len..=word_len-min_len {
        let subword1 = String::from_iter(word.chars().take(len));
        if words.binary_search(&subword1.as_str()).is_ok() {
            let subword2 = String::from_iter(word.chars().skip(len));
            if words.binary_search(&subword2.as_str()).is_ok() {
                return 1;
            }
        }
    }
    return 0;
}

#[test]
fn subwords() {
    let call_ext = || find_subwords_ext(&WORDS_PLAIN, 6, 2);
    for func in [find_subwords_fast, call_ext] {
        let time = std::time::Instant::now();
        let set_count = func();
        println!("sw time: {:.2?}\n", time.elapsed());
        
        assert_eq!(set_count, 14276);
    }
}
