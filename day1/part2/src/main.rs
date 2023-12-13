use std::{collections::BTreeMap, fs};

fn main() {
    let file = fs::read_to_string("../input").unwrap();
    let mut sum = 0;
    let numbers = BTreeMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    let mut digits: BTreeMap<usize, char>;
    for line in file.split('\n') {
        digits = BTreeMap::new();
        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                digits.insert(i, c);
            }
        }
        for key in numbers.keys() {
            // if let Some(i) = line.find(key) {
            //     digits.insert(i, numbers.get(key).unwrap().clone());
            // }
            line.match_indices(key)
                .map(|(i, _)| digits.insert(i, numbers.get(key).unwrap().clone()))
                .count();
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
