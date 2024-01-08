mod bwt;
mod min_runs_test;
mod cmp_by_spec_char;
mod count_runs;
mod rotate;
mod suffix_tree;
mod min_runs;

use std::collections::HashSet;

use crate::bwt::bwt;
use crate::min_runs::min_runs;
use crate::min_runs::min_runs_label;
use crate::count_runs::count_runs;

const START_OF_TEXT: char = '\u{02}';
const END_OF_TEXT: char = '\u{03}';

fn main() {
    println!("Enter input string: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    println!("Input String: {}", input);
    
    println!("Enter label string (for no label enter N): ");
    let mut label = String::new();
    std::io::stdin().read_line(&mut label).unwrap();
    let label = label.trim();
    if label == "N" {
        println!("No label string entered, using input string as label");

        let mut alphabet: Vec<_> = input.chars().collect::<HashSet<_>>().into_iter().collect();
        alphabet.sort_unstable();
        println!("Alphabet: {:?}", alphabet);
        
        let bwt = bwt(&input);
        println!("BWT: {}", bwt.replace(START_OF_TEXT, "^").replace(END_OF_TEXT, "|"));
        println!("Number of runs with BWT: {}", count_runs(&*bwt));
        
        println!("Minimum runs: {}",min_runs(input));
        
    }else {
        println!("Label String: {}", label);

        let mut alphabet: Vec<_> = label.chars().collect::<HashSet<_>>().into_iter().collect();
        alphabet.sort_unstable();
        println!("Alphabet: {:?}", alphabet);

        println!("Minimum runs for label: {}",min_runs_label(input, label));
    }
    
}


