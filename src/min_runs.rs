use std::collections::{HashSet, HashMap};

use itertools::Itertools;

use crate::suffix_tree::{SuffixTree, Node};

pub fn min_runs(input: &str) -> usize {
    
    let alphabet: Vec<_> = input.chars().collect::<HashSet<_>>().into_iter().collect();
    let mut dp: HashMap<(usize, usize, usize), usize> = HashMap::new();
    let tree = SuffixTree::new(input);
    let mut length = 0;

    

    find_minimal_runs(&tree, tree.root(), &alphabet, &mut dp, &mut length, &input);

    let mut result = usize::MAX;
    for i in 0..alphabet.len() {
        for j in 0..alphabet.len() {
            let current = *dp.get(&(tree.root().id(), i, j)).unwrap();
            if result > current {
                result = current;
            }
        }
    }
    
    return result;
    
}

fn find_minimal_runs(
    tree: &SuffixTree<'_>,
    node: &Node,
    alphabet: &Vec<char>,
    dp: &mut HashMap<(usize, usize, usize), usize>,
    length: &mut usize,
    text: &str,
) -> usize {

    let mut current_length = *length + node.len() as usize;
    let children: Vec<_> = node.children().collect();

    // If node is a leaf
    if children.is_empty() {

        //Get the label of the leaf (character just before suffix) == bw(L)
        let label_char: char;
        if (node.suffixes()[0] as usize) == 0 {
            label_char = text.chars().last().unwrap();
        } else {
            label_char = text.chars().nth((node.suffixes()[0] as usize) - 1).unwrap();
        }
        let label_index = alphabet.iter().position(|&c| c == label_char).unwrap();

        //For each leaf L, it is ρ(L, ci, c j) = 1 if ci = c j = bw(L) and ρ(L, ci, c j) = ∞ otherwise.
        for i in 0..alphabet.len() {
            for j in 0..alphabet.len() {
                dp.insert(
                    (node.id(), i, j),
                    if i == label_index && j == label_index {
                        1
                    } else {
                        usize::MAX
                    },
                );
            }
        }
        return 1;
    }

    // If node is internal
    else {

        // First make sure that all children have been processed and get their ids for easier access
        let mut children_ids: Vec<usize> = Vec::new();
        for &child in &children {
            find_minimal_runs(tree, child, alphabet, dp, &mut current_length, text);
            children_ids.push(child.id());
        }

        let mut permutation_results:HashMap<(usize, usize), usize> = HashMap::new();
        for i in 0..alphabet.len() {
            for j in 0..alphabet.len() {
                permutation_results
                    .insert((i, j), usize::MAX);
            }
        }
        
        let mut dp_permutation: HashMap<(usize, usize, usize), usize> = HashMap::new();
        let mut previous_permutation: Vec<&usize> = Vec::new();
        let mut first_permutation = true;
        
        // Create all permutation of children_ids
        for p in children_ids.iter().sorted().permutations(children.len()) {
            
            let mut change_in_permutation = 0;
            if first_permutation {
                first_permutation = false;
                previous_permutation = p.clone();
            }else {
                while p[change_in_permutation] == previous_permutation[change_in_permutation] {
                    change_in_permutation += 1;
                }
                previous_permutation = p.clone();
            }

            if change_in_permutation == 0 {
                //Mπ [1, l, m] = ρ(w1, l, m)
                for i in 0..alphabet.len() {
                    for j in 0..alphabet.len() {
                        dp_permutation.insert((0, i, j), *dp.get(&(*p[0], i, j)).unwrap());
                    }
                }
                change_in_permutation += 1;
            }
            

            //Mπ [k, l, m] = min(i,j) {Mπ [k − 1, l, i] + ρ(wk, j, m) − δi,j}
            //where δij = 1 if i = j and 0 otherwise

            for k in change_in_permutation..children.len() {
                for l in 0..alphabet.len() {
                    for m in 0..alphabet.len() {
                        let mut min_runs = usize::MAX;

                        for i in 0..alphabet.len() {
                            for j in 0..alphabet.len() {
                                let previous_in_permutation = *dp_permutation.get(&(k-1, l, i)).unwrap_or(&42);
                                let current_in_permutation = *dp.get(&(*p[k], j, m)).unwrap();
                                

                                if previous_in_permutation == usize::MAX
                                    || current_in_permutation == usize::MAX
                                {
                                    continue;
                                }

                                let current_runs = previous_in_permutation + current_in_permutation
                                    - if i == j { 1 } else { 0 };
                                if min_runs > current_runs {
                                    min_runs = current_runs;
                                }
                            }
                        }

                        dp_permutation.insert((k, l, m), min_runs);
                    }
                }
            }

            // take only minimum over all possible alphabet ordering π
            for i in 0..alphabet.len() {
                for j in 0..alphabet.len() {
                    if permutation_results.get(&(i, j)).unwrap_or(&usize::MAX) > dp_permutation.get(&(p.len()-1, i, j)).unwrap_or(&42) {
                        permutation_results.insert((i, j),*dp_permutation.get(&(p.len()-1, i, j)).unwrap());
                    }
                }
            }
        }
        
        for i in 0..alphabet.len() {
            for j in 0..alphabet.len() {
                dp.insert((node.id(), i, j), *permutation_results.get(&(i, j)).unwrap());
            }
        }

        return 1;
    }
}

/*
fn print_dp(dp: &HashMap<(usize, usize, usize), usize>, x: &Vec<char>) {
    for ((node_id, i, j), v) in dp {
        let char_i = x[*i];
        let char_j = x[*j];
        println!(
            "Node ID: {}, Chars: ({}, {}), Min Runs: {}",
            node_id, char_i, char_j, v
        );
    }
}
*/

