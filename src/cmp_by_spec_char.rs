use std::cmp::Ordering;

const START_OF_TEXT: char = '\u{02}';
const END_OF_TEXT: char = '\u{03}';


pub fn cmp_by_spec_char(a: &str, b: &str) -> Ordering {
    let mut a_iter = a.chars();
    let mut b_iter = b.chars();

    while let (Some(a_char), Some(b_char)) = (a_iter.next(), b_iter.next()) {
        if a_char != b_char {
            return if (a_char == END_OF_TEXT || a_char == START_OF_TEXT) && !(b_char == END_OF_TEXT || b_char == START_OF_TEXT) {
                Ordering::Less
            } else if (b_char == END_OF_TEXT || b_char == START_OF_TEXT) && !(a_char == END_OF_TEXT || a_char == START_OF_TEXT) {
                Ordering::Greater
            } else {
                a_char.cmp(&b_char)
            }
        }
    }

    return if a.len() > b.len() {
        Ordering::Greater
    } else if a.len() < b.len() {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}