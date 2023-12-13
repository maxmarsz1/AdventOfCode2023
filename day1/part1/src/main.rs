use std::{collections::BTreeMap, fs};

fn main() {
    let file = fs::read_to_string("../input").unwrap();
    let mut sum = 0;
    let mut digits: BTreeMap<usize, char>;
    for line in file.split('\n') {
        digits = BTreeMap::new();
        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                digits.insert(i, c);
            }
        }
        sum += digits.iter().next().unwrap().1.to_digit(10).unwrap() * 10
            + digits.iter().next_back().unwrap().1.to_digit(10).unwrap();
        println!("{line}");
        println!("{digits:?}");
        println!(
            "{}{}",
            digits.iter().next().unwrap().1,
            digits.iter().next_back().unwrap().1
        );
    }
    println!("{sum}");
}
