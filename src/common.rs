use std::path::{Path, PathBuf};

pub type WordsStorage = std::collections::HashSet<&'static str>;
pub type WordsStoragePlain = Vec<&'static str>;

// Make file pathes and load data files in memory
lazy_static::lazy_static! {
    static ref CARGO_MANIFEST_DIR: &'static Path = Path::new(env!("CARGO_MANIFEST_DIR"));
    pub static ref DATA_DIR: PathBuf = CARGO_MANIFEST_DIR.join("data");
    pub static ref WORDS_FILE: PathBuf = DATA_DIR.join("wordlist.txt");
    pub static ref WORDS_DATA: String = {
        std::fs::read_to_string(&*WORDS_FILE).unwrap()
    };
    pub static ref WORDS: WordsStorage = {
        WORDS_DATA.lines().collect()
    };
    pub static ref WORDS_PLAIN: WordsStoragePlain = {
        WORDS_DATA.lines().collect()
    };
}
