fn main() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = String::from_utf8(std::fs::read("inputs/01.txt")?)?;

    let calorie_sums: Vec<usize> = inputs
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .fold(0, |acc, v| acc + v.parse::<usize>().unwrap_or(0))
        })
        .collect();

    let mut largest = 0;
    for sum in calorie_sums {
        if sum > largest {
            largest = sum
        }
    }

    println!("largest: {}", largest);

    Ok(())
}
