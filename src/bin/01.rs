fn main() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = String::from_utf8(std::fs::read("inputs/01.txt")?)?;

    let largest = inputs
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .fold(0, |acc, v| acc + v.parse::<usize>().unwrap_or(0))
        })
        .max()
        .unwrap();

    println!("largest: {largest}");

    Ok(())
}
