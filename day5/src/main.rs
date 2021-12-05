use either::*;
use itertools::iproduct;
use std::{collections::HashMap, io, ops::RangeInclusive};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct UVec2(u16, u16);

impl UVec2 {
    pub fn new(x: u16, y: u16) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> u16 {
        self.0
    }

    pub fn y(&self) -> u16 {
        self.1
    }
}

#[derive(Clone, Copy, Debug)]
struct LineSegment {
    start: UVec2,
    end: UVec2,
    is_diagonal: bool,
}

impl LineSegment {
    pub fn new(start: UVec2, end: UVec2) -> Self {
        Self {
            start,
            end,
            is_diagonal: start.x() != end.x() && start.y() != end.y(),
        }
    }

    pub fn active_coordinates(&self) -> (RangeInclusive<u16>, RangeInclusive<u16>) {
        let start = self.start();
        let end = self.end();

        let range_x = if start.x() < end.x() {
            start.x()..=end.x()
        } else {
            end.x()..=start.x()
        };

        let range_y = if start.y() < end.y() {
            start.y()..=end.y()
        } else {
            end.y()..=start.y()
        };

        (range_x, range_y)
    }

    pub fn end(&self) -> UVec2 {
        self.end
    }

    pub fn is_diagonal(&self) -> bool {
        self.is_diagonal
    }

    pub fn is_x_backwards(&self) -> bool {
        self.start().x() > self.end().x()
    }

    pub fn is_y_backwards(&self) -> bool {
        self.start().y() > self.end().y()
    }

    pub fn start(&self) -> UVec2 {
        self.start
    }
}

// Solutions for day 5. This is a pretty flawed solution that I went for based on my original
// assumption that Rust ranges were bi-directional.
fn main() -> io::Result<()> {
    let input = std::fs::read_to_string("assets/input.txt")?;

    let line_segments = parse_line_segments(&input);

    println!(
        "Number of dangerous areas: {}",
        find_dangerous_areas(&line_segments).len()
    );

    Ok(())
}

// Parts one and two
fn find_dangerous_areas(line_segments: &[LineSegment]) -> Vec<UVec2> {
    let mut vent_positions: HashMap<UVec2, u8> = HashMap::new();

    for line_segment in line_segments {
        let (x_range, y_range) = line_segment.active_coordinates();

        let x_range = if line_segment.is_x_backwards() {
            Left(x_range.rev())
        } else {
            Right(x_range)
        };

        let y_range = if line_segment.is_y_backwards() {
            Left(y_range.rev())
        } else {
            Right(y_range)
        };

        if line_segment.is_diagonal() {
            for (x, y) in x_range.zip(y_range) {
                let vent_count = vent_positions.entry(UVec2::new(x, y)).or_insert(0);
                *vent_count += 1;
            }
        } else {
            for (x, y) in iproduct!(x_range, y_range) {
                if line_segment.start() == UVec2::new(800, 61) {
                    println!("x: {}, y: {}", x, y);
                }
                let vent_count = vent_positions.entry(UVec2::new(x, y)).or_insert(0);
                *vent_count += 1;
            }
        }
    }

    vent_positions.retain(|_, vent_count| *vent_count > 1);
    vent_positions.into_keys().collect()
}

fn parse_line_segments(input: &str) -> Vec<LineSegment> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .flat_map(|coordinates| {
                    let coordinates: Vec<u16> =
                        coordinates.split(',').flat_map(str::parse::<u16>).collect();

                    if coordinates.len() == 2 {
                        Some(UVec2::new(coordinates[0], coordinates[1]))
                    } else {
                        None
                    }
                })
                .collect::<Vec<UVec2>>()
        })
        .flat_map(|positions| {
            if positions.len() == 2 {
                Some(LineSegment::new(positions[0], positions[1]))
            } else {
                None
            }
        })
        .collect()
}
