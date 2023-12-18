use crate::cmp_by_spec_char::cmp_by_spec_char;
use crate::rotate::rotate;

// const START_OF_TEXT: char = '\u{02}';
// const END_OF_TEXT: char = '\u{03}';

pub fn bwt(input: &str) -> String {
    let mut table: Vec<String> = vec![];

    //let mut input_string = format!("{}{}{}", START_OF_TEXT, input, END_OF_TEXT);
    let mut input_string = format!("{}", input);
    

    //all possible rotations probably will be updated
    let mut i = input_string.len();

    while i > 0 {
        table.push(
            format!("{}",input_string)
        );
        input_string = rotate(&*input_string);
        i = i-1;
    }

    table.sort_by(|a, b| cmp_by_spec_char(&a, &b));


    table
        .iter()
        .map(|s| s.chars().nth_back(0).unwrap())
        .collect::<String>()
}