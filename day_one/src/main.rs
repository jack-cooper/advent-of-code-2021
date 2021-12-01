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

    println!("Depths greater than previous: {}", part_one(&depths));
    println!(
        "Three measurement sliding window sums greater than previous: {}",
        part_two(&depths)
    );

    Ok(())
}

#[allow(dead_code)]
fn part_one(depths: &[u32]) -> u32 {
    depths
        .windows(2)
        .fold(0, |depths_greater_than_previous, depths| {
            if depths[1] > depths[0] {
                depths_greater_than_previous + 1
            } else {
                depths_greater_than_previous
            }
        })
}

fn part_two(depths: &[u32]) -> u32 {
    depths
        .windows(4)
        .skip(1)
        .fold(0, |depths_greater_than_previous, depths| {
            if depths[1..4].iter().sum::<u32>() > depths[0..3].iter().sum() {
                depths_greater_than_previous + 1
            } else {
                depths_greater_than_previous
            }
        })
}
