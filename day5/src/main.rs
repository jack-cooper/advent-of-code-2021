use std::{collections::HashMap, io, ops::RangeInclusive};

use itertools::iproduct;

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
struct LineSegment(UVec2, UVec2);

impl LineSegment {
    pub fn new(start: UVec2, end: UVec2) -> Option<Self> {
        if start.x() != end.x() && start.y() != end.y() {
            None
        } else {
            Some(Self(start, end))
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
        self.1
    }

    pub fn start(&self) -> UVec2 {
        self.0
    }
}

fn main() -> io::Result<()> {
    let input = std::fs::read_to_string("assets/input.txt")?;

    let line_segments = parse_line_segments(&input);

    println!(
        "Number of dangerous areas: {}",
        find_dangerous_areas(&line_segments).len()
    );

    Ok(())
}

// Part one
fn find_dangerous_areas(line_segments: &[LineSegment]) -> Vec<UVec2> {
    let mut vent_positions: HashMap<UVec2, u8> = HashMap::new();

    for line_segment in line_segments {
        let (x_range, y_range) = line_segment.active_coordinates();
        dbg!(x_range.clone(), y_range.clone());

        for (x, y) in iproduct!(x_range, y_range) {
            let vent_count = vent_positions.entry(UVec2::new(x, y)).or_insert(0);
            *vent_count += 1;
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
                LineSegment::new(positions[0], positions[1])
            } else {
                None
            }
        })
        .collect()
}
