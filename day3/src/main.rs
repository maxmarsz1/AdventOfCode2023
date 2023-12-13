use regex::Regex;
use std::{collections::BTreeMap, fs};

#[derive(Debug)]
struct Symbol {
    col: usize,
    sym: char,
}

impl Symbol {
    fn get_gear_ratio(&self, row: usize, numbers: &BTreeMap<usize, Vec<Number>>) -> i32 {
        let mut neighbours: Vec<&Number> = Vec::new();
        for y in row - 1..row + 2 {
            if let Some(cols) = numbers.get(&y) {
                let filtered: Vec<&Number> = cols
                    .iter()
                    .filter(|n| {
                        (self.col - 1..self.col + 2).contains(&n.coord.col)
                            || (self.col - 1..self.col + 2).contains(&(n.coord.col_end - 1))
                    })
                    .collect();
                for num in filtered {
                    if !neighbours.contains(&num) {
                        neighbours.push(num);
                    }
                }
                println!("Symbol: {} at {}x{}", self.sym, row, self.col);
                println!("{:?}", neighbours);
            }
        }
        let mut power = 1;
        if neighbours.len() == 2 {
            for n in neighbours {
                power *= n.value;
            }
            return power;
        }
        0
    }
}

#[derive(Debug)]
struct Coord {
    row: usize,
    col: usize,
    col_end: usize,
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

#[derive(Debug)]
struct Number {
    value: i32,
    coord: Coord,
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.coord == other.coord
    }
}

impl Number {
    fn is_adjacent_to_sym(&self, row: usize, symbols: &BTreeMap<usize, Vec<Symbol>>) -> bool {
        let ixstart = if self.coord.col == 0 {
            0
        } else {
            self.coord.col - 1
        };
        let ixend = self.coord.col + self.value.to_string().len();
        let iystart = if row == 0 { 0 } else { row - 1 };
        let iyend = row + 1;
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
    // let mut numbers: Vec<Number> = Vec::new();
    let mut numbers1: BTreeMap<usize, Vec<Number>> = BTreeMap::new();
    let mut sum = 0;

    //Collecting symbols cords
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if !ch.is_ascii_digit() && ch != '.' {
                let sym = Symbol { col: c, sym: ch };

                symbols.entry(r).or_default();
                symbols.get_mut(&r).unwrap().push(sym);
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
            println!("{}", num.is_adjacent_to_sym(r, &symbols));
            if num.is_adjacent_to_sym(r, &symbols) {
                sum += num.value;
            }
            numbers1.entry(r).or_default();
            numbers1.get_mut(&r).unwrap().push(num);
        }
    }
    let mut power = 0;
    for (row, syms) in &symbols {
        for sym in syms {
            if sym.sym == '*' {
                power += sym.get_gear_ratio(*row, &numbers1);
            }
        }
    }
    // println!("{symbols:?}");
    // println!("{numbers1:?}");
    println!("Part1: {sum}");
    println!("Part2: {power}");
}
