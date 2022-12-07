const WINDOW_SIZE: usize = 4;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = String::from_utf8(std::fs::read("inputs/06.txt")?)?;

    let windows = input.as_bytes().windows(WINDOW_SIZE);

    for (idx, window) in windows.enumerate() {
        if window[0] != window[1]
            && window[0] != window[2]
            && window[0] != window[3]
            && window[1] != window[2]
            && window[1] != window[3]
            && window[2] != window[3]
        {
            println!("output: {}", idx + WINDOW_SIZE);
            break;
        }
    }

    Ok(())
}
