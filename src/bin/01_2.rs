fn main() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = String::from_utf8(std::fs::read("inputs/01.txt")?)?;

    let mut calorie_sums: Vec<usize> = inputs
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|v| v.parse::<usize>().unwrap_or(0))
                .sum()
        })
        .collect();
    calorie_sums.sort_unstable();
    let total: usize = calorie_sums.iter().rev().take(3).sum();
    println!("total: {total}");

    Ok(())
}
