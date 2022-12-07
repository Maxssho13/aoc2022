use std::collections::HashSet;

const WINDOW_SIZE: usize = 14;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = String::from_utf8(std::fs::read("inputs/06.txt")?)?;

    let windows = input.as_bytes().windows(WINDOW_SIZE).enumerate();

    let mut set = HashSet::with_capacity(WINDOW_SIZE);
    // this could be made more efficient by only checking the incoming character in the window
    // but this works so whatever.
    'outer: for (idx, window) in windows {
        for character in window {
            if set.contains(character) {
                set.clear();
                continue 'outer;
            }
            set.insert(character);
        }
        println!("output: {}", idx + WINDOW_SIZE);
        break;
    }

    Ok(())
}
