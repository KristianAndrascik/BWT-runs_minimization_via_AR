pub fn rotate(str: &str) -> String {
    let mut str_vec: Vec<char> = str.chars().collect();
    str_vec.rotate_right(1);
    str_vec.iter().collect()
}
