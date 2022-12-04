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
    for rucksack in input.lines() {
        let compartment1 = &rucksack[..rucksack.len() / 2];
        let dict: HashSet<char> = HashSet::from_iter(compartment1.chars());

        let compartment2 = &rucksack[rucksack.len() / 2..];
        for character in compartment2.chars() {
            if dict.contains(&character) {
                output += convert_priority(character);
                break;
            }
        }
    }

    println!("output: {output}");
    Ok(())
}
