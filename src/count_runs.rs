pub fn count_runs(bwt: &str) -> i32 {
    let mut run_count = 1;
    let mut prev_char = bwt.chars().next().unwrap();

    for c in bwt.chars().skip(1) {
        if c != prev_char {
            run_count += 1;
        }
        prev_char = c;
    }

    return run_count;
}


