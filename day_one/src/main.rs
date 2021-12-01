use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let input = File::open("day_one/assets/input.txt")?;

    let depths: Vec<u32> = BufReader::new(input)
        .lines()
        .flat_map(|line| line.map(|value| value.parse::<u32>()))
        .flatten()
        .collect();

    let depths_greater_than_previous =
        depths
            .windows(2)
            .fold(0, |depths_greater_than_previous, depths| {
                if depths[1] > depths[0] {
                    depths_greater_than_previous + 1
                } else {
                    depths_greater_than_previous
                }
            });

    println!(
        "Depths greater than previous: {}",
        depths_greater_than_previous
    );

    Ok(())
}
