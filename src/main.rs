mod common;
mod kata1_pricing;
mod kata2_chop;
mod kata4_munging;
mod kata5_bloom;
mod kata6_anagrams;
mod kata8_objectives;

fn main() {
    kata5_bloom::print_stats();

    println!("\nRun 'cargo test' to execute all samples.");
}
