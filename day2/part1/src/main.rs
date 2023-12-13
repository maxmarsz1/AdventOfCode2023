use std::{collections::HashMap, fs};

fn main() {
    let max_values: HashMap<&str, i32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut game_values: HashMap<&str, i32>;
    let mut sum = 0;
    let mut power_sum = 0;
    let input = fs::read_to_string("../input").unwrap();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let game_id = &line[5..line.find(':').unwrap()].parse::<i32>().unwrap();
        println!("Game id: {game_id}");
        game_values = HashMap::new();
        let games = line[line.find(':').unwrap() + 2..].split("; ");
        for game in games {
            // println!("{game:?}");
            let cubes = game.split(", ");
            for cube in cubes {
                // println!("{cube}");
                let count = cube[..cube.find(' ').unwrap()].parse::<i32>().unwrap();
                let color = &cube[cube.find(' ').unwrap() + 1..];
                let mut value = game_values.entry(color).or_insert(count);
                if *value < count {
                    value = game_values.get_mut(color).unwrap();
                    *value = count;
                }
            }
        }
        println!("{game_values:?}");

        let mut higher = false;
        let mut game_power = 1;
        for &key in game_values.keys() {
            if game_values.get(key).unwrap() > max_values.get(key).unwrap() {
                higher = true;
            }
            game_power = game_power * game_values.get(key).unwrap();
        }
        println!("{game_power}");
        power_sum += game_power;
        if !higher {
            sum += game_id;
        }
    }
    println!("Part1: {sum}");
    println!("Part2: {power_sum}");
}
