use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    num::ParseIntError,
};

// Answers for https://adventofcode.com/2021/day/3
fn main() -> io::Result<()> {
    let input = File::open("assets/input.txt")?;

    let power_consumptions: Vec<u16> = BufReader::new(input)
        .lines()
        .flat_map(|consumption| consumption.map(|consumption| u16::from_str_radix(&consumption, 2)))
        .flatten()
        .collect();

    println!("{:?}", calculate_power_consumption(power_consumptions));
    Ok(())
}

// Part one - This is absolutely horrible code but I thought it'd be more in the spirit of the challenge to do a whole lot of bitwise operations
fn calculate_power_consumption(power_consumptions: Vec<u16>) -> Result<u32, ParseIntError> {
    let gamma_rate = power_consumptions
        .iter()
        .fold([0_i16; 12], |mut bit_counts, consumption| {
            for (offset, bit_count) in bit_counts.iter_mut().enumerate() {
                if (consumption & (1 << offset)) == 1 << offset {
                    *bit_count += 1;
                } else {
                    *bit_count -= 1;
                }
            }
            bit_counts
        })
        .iter()
        .enumerate()
        .rev()
        .fold(0_u16, |gamma_rate, (offset, &bit_count)| {
            gamma_rate + if bit_count > 0 { 1 << offset } else { 0 }
        });

    let epsilon_rate = !gamma_rate & 0b0000111111111111;

    Ok(gamma_rate as u32 * epsilon_rate as u32)
}
