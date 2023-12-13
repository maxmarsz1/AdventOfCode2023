use regex::Regex;
use std::{collections::BTreeMap, fs};

#[derive(Debug)]
struct Symbol {
    col: usize,
    sym: char,
}

impl Symbol {
    // fn is_adjacent_to_two_num(&self, row: usize, numbers: &Vec<Number>) -> bool {
    //     for i in row - 1..row + 2 {

    //     }
    // }
}

#[derive(Debug)]
struct Coord {
    row: usize,
    col: usize,
    col_end: usize,
}

#[derive(Debug)]
struct Number {
    value: i32,
    coord: Coord,
}

impl Number {
    fn is_adjacent_to_sym(&self, symbols: &BTreeMap<usize, Vec<Symbol>>) -> bool {
        let ixstart = if self.coord.col == 0 {
            0
        } else {
            self.coord.col - 1
        };
        let ixend = self.coord.col + self.value.to_string().len();
        let iystart = if self.coord.row == 0 {
            0
        } else {
            self.coord.row - 1
        };
        let iyend = self.coord.row + 1;
        println!("Searched area is: {ixstart}-{ixend}x{iystart}-{iyend}");
        for i in iystart..iyend + 1 {
            if let Some(cols) = symbols.get(&i) {
                if cols
                    .iter()
                    .filter(|s| s.col >= ixstart && s.col <= ixend)
                    .count()
                    > 0
                {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut symbols: BTreeMap<usize, Vec<Symbol>> = BTreeMap::new();
    let mut numbers: Vec<Number> = Vec::new();
    let mut sum = 0;

    //Collecting symbols cords
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if !ch.is_ascii_digit() && ch != '.' {
                symbols.entry(r).or_default();
                symbols
                    .get_mut(&r)
                    .unwrap()
                    .push(Symbol { col: c, sym: ch });
                println!("{ch}");
            }
        }
    }
    let re = Regex::new(r"[0-9][0-9]*").unwrap();
    for (r, line) in input.lines().enumerate() {
        for needle in re.find_iter(line) {
            let num = needle.as_str().parse::<i32>().unwrap();
            println!("{num} at ({r}, {})", needle.start());
            let num = Number {
                value: num,
                coord: Coord {
                    row: r,
                    col: needle.start(),
                    col_end: needle.end(),
                },
            };
            println!("{}", num.is_adjacent_to_sym(&symbols));
            if num.is_adjacent_to_sym(&symbols) {
                sum += num.value;
            }
            numbers.push(num);
        }
    }
    println!("{symbols:?}");
    println!("{numbers:?}");
    println!("{sum}");
}
