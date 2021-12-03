use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

struct Instruction {
    direction: String,
    magnitude: u32,
}

impl Instruction {
    pub fn new(direction: String, magnitude: u32) -> Self {
        Self {
            direction,
            magnitude,
        }
    }
}

fn main() -> io::Result<()> {
    let input = File::open("assets/input.txt")?;

    let instructions: Vec<Instruction> = BufReader::new(input)
        .lines()
        .flat_map(|instruction| {
            if let Ok(instruction) = instruction {
                let mut instruction_words = instruction.split(' ');
                let direction = instruction_words.next();
                let magnitude = instruction_words
                    .next()
                    .map(|magnitude| magnitude.parse::<u32>());

                match direction {
                    Some(direction) => match magnitude {
                        Some(Ok(magnitude)) => {
                            Some(Instruction::new(direction.to_string(), magnitude))
                        }
                        Some(Err(_)) => None,
                        None => None,
                    },
                    None => None,
                }
            } else {
                None
            }
        })
        .collect();

    println!(
        "horizontal distance * depth = {}",
        distance_depth_product(&instructions)
    );

    println!(
        "When accounting for aim, horizontal distance * depth = {}",
        distance_depth_product_with_aim(&instructions)
    );

    Ok(())
}

const DOWN: &str = "down";
const FORWARD: &str = "forward";
const UP: &str = "up";

// Part one
fn distance_depth_product(instructions: &[Instruction]) -> u32 {
    let mut horizontal_distance: u32 = 0;
    let mut depth = 0;

    for instruction in instructions {
        match instruction.direction.as_str() {
            DOWN => depth += instruction.magnitude,
            FORWARD => horizontal_distance += instruction.magnitude,
            UP => depth -= instruction.magnitude,
            _ => panic!("Abort! Abort! Unrecognised instruction detected"),
        }
    }

    horizontal_distance * depth
}

// Part two
fn distance_depth_product_with_aim(instructions: &[Instruction]) -> u32 {
    let mut aim = 0;
    let mut horizontal_distance: u32 = 0;
    let mut depth = 0;

    for instruction in instructions {
        match instruction.direction.as_str() {
            DOWN => aim += instruction.magnitude,
            FORWARD => {
                horizontal_distance += instruction.magnitude;
                depth += instruction.magnitude * aim;
            }
            UP => aim -= instruction.magnitude,
            _ => panic!("Abort! Abort! Unrecognised instruction detected"),
        }
    }

    horizontal_distance * depth
}
