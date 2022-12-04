#![feature(iter_array_chunks)]
use std::collections::HashSet;

fn convert_priority(c: char) -> i32 {
    if c.is_uppercase() {
        c as i32 - 38
    } else {
        c as i32 - 96
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = String::from_utf8(std::fs::read("inputs/03.txt")?)?;

    let mut output = 0;
    for [rucksack1, rucksack2, rucksack3] in input.lines().array_chunks() {
        let dict: HashSet<char> = HashSet::from_iter(rucksack1.chars());

        let mut dict2: HashSet<char> = HashSet::new();
        for character in rucksack2.chars() {
            if dict.contains(&character) {
                dict2.insert(character);
            }
        }

        for character in rucksack3.chars() {
            if dict2.contains(&character) {
                output += convert_priority(character);
                break;
            }
        }
    }

    println!("output: {output}");
    Ok(())
}
