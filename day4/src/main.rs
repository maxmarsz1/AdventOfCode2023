use std::{fs, collections::HashMap};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut copies: HashMap<i32, i32> = HashMap::new();
    let mut sum_of_points = 0;
    for card in input.lines(){
        let card_id = &card[card.find(' ').unwrap()..card.find(':').unwrap()]
                .trim()
                .parse::<i32>()
                .unwrap();
        println!("Game {card_id}");
        if let Some(value) = copies.get_mut(card_id) {
            *value += 1;
        } else {
            copies.insert(*card_id, 1);
        }

        let numbers: Vec<i32> = card[card.find(':').unwrap()+1..card.find('|').unwrap()-1]
                .trim()
                .split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

        let w_numbers: Vec<i32> = card[card.find('|').unwrap()+1..]
                .trim()
                .split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

        let mut points = 0;
        let mut copies_won = 0;
        println!("No scratchcards: {}", *copies.get(card_id).unwrap());
        for n in &numbers {
            if w_numbers.contains(n){
                copies_won+=1;
            }
        }
        println!("Won copies: {copies_won}");
        if copies_won > 0 {
            println!("Range: {}-{}", card_id+1, *card_id+1+copies_won);
            for i in card_id+1..*card_id+1+copies_won{
                copies.entry(i).or_insert(0);
                *copies.get_mut(&i).unwrap() += *copies.get(card_id).unwrap();
            }
        }
        
        for n in &numbers {
            if w_numbers.contains(n){
                points = if points == 0 {1} else {points*2};
            }
        }
        sum_of_points += points;
    }
    let mut no_cards = 0;
    for (_card, amount) in copies.iter() {
        no_cards += amount;
    }

    println!("{sum_of_points}");
    println!("{no_cards}");
}
