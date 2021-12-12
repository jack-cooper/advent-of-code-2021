use std::collections::HashSet;

const GRID_SIZE: usize = 10;

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("assets/input.txt")?;

    let mut octopi: Vec<u32> = input
        .lines()
        .flat_map(str::chars)
        .flat_map(|digit| digit.to_digit(10))
        .collect();

    let total_flashes = simulate_steps(100, &mut octopi);

    dbg!(total_flashes);

    let mut octopi: Vec<u32> = input
        .lines()
        .flat_map(str::chars)
        .flat_map(|digit| digit.to_digit(10))
        .collect();

    let steps_until_synced = simulate_until_synchronised(&mut octopi);

    dbg!(steps_until_synced);

    Ok(())
}

fn get_adjacent_indices(index: usize) -> Vec<usize> {
    let mut adjacent_indices: Vec<usize> = Vec::with_capacity(8);

    let is_top_row = index < GRID_SIZE;
    let is_bottom_row = index >= GRID_SIZE.pow(2) - GRID_SIZE;
    let is_left_column = index % GRID_SIZE == 0;
    let is_right_column = index % GRID_SIZE == GRID_SIZE - 1;

    if !is_top_row {
        adjacent_indices.push(index - GRID_SIZE);

        if !is_left_column {
            adjacent_indices.push(index - GRID_SIZE - 1);
        }
        if !is_right_column {
            adjacent_indices.push(index - GRID_SIZE + 1);
        }
    }

    if !is_bottom_row {
        adjacent_indices.push(index + GRID_SIZE);

        if !is_left_column {
            adjacent_indices.push(index + GRID_SIZE - 1);
        }
        if !is_right_column {
            adjacent_indices.push(index + GRID_SIZE + 1);
        }
    }

    if !is_left_column {
        adjacent_indices.push(index - 1);
    }

    if !is_right_column {
        adjacent_indices.push(index + 1);
    }

    adjacent_indices
}

fn simulate_steps(steps: u16, octopi: &mut [u32]) -> u32 {
    let mut total_flashes = 0;

    for _ in 0..steps {
        let mut flashed_octopus_indices: HashSet<usize> = HashSet::with_capacity(GRID_SIZE.pow(2));
        let mut flashing_octopus_indices: Vec<usize> = Vec::with_capacity(GRID_SIZE.pow(2));

        for (index, octopus) in octopi.iter_mut().enumerate() {
            *octopus += 1;

            if *octopus == 10 {
                flashing_octopus_indices.push(index);
            }
        }

        while !flashing_octopus_indices.is_empty() {
            let mut new_flashing_octopus_indices: Vec<usize> = Vec::with_capacity(GRID_SIZE.pow(2));

            for &index in &flashing_octopus_indices {
                flashed_octopus_indices.insert(index);
                total_flashes += 1;

                for adjacent_index in get_adjacent_indices(index) {
                    octopi[adjacent_index] += 1;

                    if octopi[adjacent_index] == 10 {
                        new_flashing_octopus_indices.push(adjacent_index);
                    }
                }
            }

            flashing_octopus_indices = new_flashing_octopus_indices;

            flashing_octopus_indices.retain(|index| !flashed_octopus_indices.contains(index));
        }

        for &index in &flashed_octopus_indices {
            octopi[index] = 0;
        }
    }

    total_flashes
}

fn simulate_until_synchronised(octopi: &mut [u32]) -> u32 {
    let mut steps = 0;

    'steps: loop {
        steps += 1;

        let mut flashed_octopus_indices: HashSet<usize> = HashSet::with_capacity(GRID_SIZE.pow(2));
        let mut flashing_octopus_indices: Vec<usize> = Vec::with_capacity(GRID_SIZE.pow(2));

        for (index, octopus) in octopi.iter_mut().enumerate() {
            *octopus += 1;

            if *octopus == 10 {
                flashing_octopus_indices.push(index);
            }
        }

        while !flashing_octopus_indices.is_empty() {
            let mut new_flashing_octopus_indices: Vec<usize> = Vec::with_capacity(GRID_SIZE.pow(2));

            for &index in &flashing_octopus_indices {
                flashed_octopus_indices.insert(index);

                for adjacent_index in get_adjacent_indices(index) {
                    octopi[adjacent_index] += 1;

                    if octopi[adjacent_index] == 10 {
                        new_flashing_octopus_indices.push(adjacent_index);
                    }
                }
            }

            if flashed_octopus_indices.len() == GRID_SIZE.pow(2) {
                break 'steps steps;
            }
            flashing_octopus_indices = new_flashing_octopus_indices;

            flashing_octopus_indices.retain(|index| !flashed_octopus_indices.contains(index));
        }

        for &index in &flashed_octopus_indices {
            octopi[index] = 0;
        }
    }
}
