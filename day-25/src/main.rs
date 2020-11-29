use std::fs;

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

const INPUT: &str = "input/data.txt";

fn main() -> Result<()> {
    let contents = fs::read_to_string(INPUT)?;

    dbg!(contents);

    Ok(())
}
