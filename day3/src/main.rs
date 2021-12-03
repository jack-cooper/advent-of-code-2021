use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    num::ParseIntError,
};

// Answers for https://adventofcode.com/2021/day/3
fn main() -> io::Result<()> {
    let input = File::open("assets/input.txt")?;

    let diagnostic_readings: Vec<u16> = BufReader::new(input)
        .lines()
        .flat_map(|reading| reading.map(|reading| u16::from_str_radix(&reading, 2)))
        .flatten()
        .collect();

    println!("{:?}", calculate_power_consumption(&diagnostic_readings));
    println!("{}", calcaulate_life_support_rating(&diagnostic_readings));
    Ok(())
}

fn calculate_bit_counts(diagnostic_readings: &[u16]) -> [i16; 12] {
    diagnostic_readings
        .iter()
        .fold([0_i16; 12], |mut bit_counts, reading| {
            for (offset, bit_count) in bit_counts.iter_mut().enumerate() {
                if (reading & (1 << offset)) == 1 << offset {
                    *bit_count += 1;
                } else {
                    *bit_count -= 1;
                }
            }
            bit_counts
        })
}

#[derive(Debug, PartialEq)]
enum LifeSupportSubsection {
    OxygenGenerator,
    CarbonDioxideScrubber,
}

// Answer for part two. Probably the worst code I've written in years
fn calcaulate_life_support_rating(diagnostic_readings: &[u16]) -> u32 {
    let oxygen_generator_rating = calculate_life_support_subsection(
        diagnostic_readings,
        0,
        LifeSupportSubsection::OxygenGenerator,
    );

    let carbon_dioxide_scrubber_rating = calculate_life_support_subsection(
        diagnostic_readings,
        0,
        LifeSupportSubsection::CarbonDioxideScrubber,
    );

    oxygen_generator_rating as u32 * carbon_dioxide_scrubber_rating as u32
}

fn calculate_life_support_subsection(
    diagnostic_readings: &[u16],
    offset: usize,
    subsection: LifeSupportSubsection,
) -> u16 {
    let mut most_common_bits =
        calculate_bit_counts(diagnostic_readings).map(|bit_count| match bit_count.signum() {
            0 | 1 => 1,
            -1 => 0,
            _ => panic!("signum is not behaving as expected"),
        });

    most_common_bits.reverse();

    let filter_closure: Box<dyn Fn(&u16) -> bool> =
        if subsection == LifeSupportSubsection::OxygenGenerator {
            Box::new(|reading| {
                (reading & (1 << (11 - offset))) >> (11 - offset) == most_common_bits[offset] as u16
            })
        } else {
            Box::new(|reading| {
                (reading & (1 << (11 - offset))) >> (11 - offset) != most_common_bits[offset] as u16
            })
        };

    let filtered_diagnostic_readings: Vec<u16> = diagnostic_readings
        .to_owned()
        .into_iter()
        .filter(filter_closure)
        .collect();

    println!("Filtered length: {}", filtered_diagnostic_readings.len());

    if filtered_diagnostic_readings.len() == 1 {
        filtered_diagnostic_readings[0]
    } else if filtered_diagnostic_readings.is_empty() {
        0
    } else {
        calculate_life_support_subsection(&filtered_diagnostic_readings, offset + 1, subsection)
    }
}

// Part one - This is absolutely horrible code but I thought it'd be more in the spirit of the challenge to do a whole lot of bitwise operations
fn calculate_power_consumption(diagnostic_readings: &[u16]) -> Result<u32, ParseIntError> {
    let gamma_rate = calculate_bit_counts(diagnostic_readings)
        .iter()
        .enumerate()
        .fold(0_u16, |gamma_rate, (offset, &bit_count)| {
            gamma_rate + if bit_count > 0 { 1 << offset } else { 0 }
        });

    let epsilon_rate = !gamma_rate & 0b0000111111111111;

    Ok(gamma_rate as u32 * epsilon_rate as u32)
}
