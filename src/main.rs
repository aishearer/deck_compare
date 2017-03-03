use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use std::cmp;

//TODO strip basic lands from file with flag
//TODO support numbers over 9

fn main() {

    let death_and_taxes = file_to_deck_map("/Users/ashearer/repos/deck_compare/decks/death_and_taxes.txt");
    let eldrazi_and_taxes = file_to_deck_map("/Users/ashearer/repos/deck_compare/decks/eldrazi_and_taxes.txt");
    let in_common = number_in_common(death_and_taxes, eldrazi_and_taxes);
    println!("{}", in_common);
}

fn file_to_deck_map(path_str: &str) -> HashMap<String, i32> {
    let path = Path::new(path_str);
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(),
                                                   why.description()),
        Ok(file) => file,
    };

    let mut card_map: HashMap<String, i32> = HashMap::new();

    let reader = BufReader::new(&file);
    for line in reader.lines() {
        let l = match line {
            Err(why) => panic!("couldn't read {}: {}", path.display(),
                               why.description()),
            Ok(l) => l,
        };
        if l.len() > 2 {
            let number_of_card = &l[0..1];
            let num = match number_of_card.parse::<i32>() {
                Err(_) => {
                    println!("!!! Warning: Couldn't figure-out how may cards you wanted here: {}  Found in file: {}", l, path.display());
                    continue;
                },
                Ok(num) => num,
            };
            let card_name = l[2..].to_string();
            card_map.insert(card_name, num);
        }
    }

    card_map
}

fn number_in_common(deck_1: HashMap<String, i32>, deck_2: HashMap<String, i32>) -> i32 {
    let mut total_in_common = 0;
    for (deck_1_card, deck_1_card_number) in deck_1 {
        match deck_2.get(&deck_1_card) {
            Some(deck_2_card_number) => {
                println!("{} {}", cmp::min(deck_1_card_number, *deck_2_card_number), deck_1_card);
                total_in_common += cmp::min(deck_1_card_number, *deck_2_card_number)
            },
            None => {},
        }
    }
    total_in_common
}
