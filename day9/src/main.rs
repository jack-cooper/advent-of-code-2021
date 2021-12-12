use std::collections::HashMap;

const LINE_LENGTH: usize = 100;

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("assets/input.txt")?;

    let heightmap: Vec<u32> = input
        .lines()
        .flat_map(str::chars)
        .flat_map(|digit| digit.to_digit(10))
        .collect();

    let low_points = get_low_points(&heightmap);

    let total_risk_level = low_points.iter().fold(0, |total_risk_level, (_, height)| {
        total_risk_level + height + 1
    });

    let basin_sizes = get_basin_sizes(&low_points, &heightmap);

    let largest_basins = basin_sizes
        .iter()
        .fold([0, 0, 0], |mut largest_basins, &basin| {
            if basin > largest_basins[0] {
                largest_basins[2] = largest_basins[1];
                largest_basins[1] = largest_basins[0];
                largest_basins[0] = basin;
            } else if basin > largest_basins[1] {
                largest_basins[2] = largest_basins[1];
                largest_basins[1] = basin;
            } else if basin > largest_basins[2] {
                largest_basins[2] = basin;
            }

            largest_basins
        });

    let largest_basin_size_product: usize = largest_basins.iter().product();

    dbg!(total_risk_level);
    dbg!(largest_basin_size_product);

    Ok(())
}

fn get_adjacent_locations(index: usize, heightmap: &[u32]) -> HashMap<usize, u32> {
    let mut adjacent_locations: HashMap<usize, u32> = HashMap::new();

    if index % LINE_LENGTH != 99 {
        let adjacent_index = index + 1;
        adjacent_locations.insert(adjacent_index, heightmap[adjacent_index]);
    }
    if index % LINE_LENGTH != 0 {
        let adjacent_index = index - 1;
        adjacent_locations.insert(adjacent_index, heightmap[adjacent_index]);
    }
    if index > 99 {
        let adjacent_index = index - 100;
        adjacent_locations.insert(adjacent_index, heightmap[adjacent_index]);
    }
    if index < heightmap.len() - 100 {
        let adjacent_index = index + 100;

        adjacent_locations.insert(adjacent_index, heightmap[adjacent_index]);
    }

    adjacent_locations
}

// Part two
fn get_basin_sizes(low_points: &HashMap<usize, u32>, heightmap: &[u32]) -> Vec<usize> {
    low_points
        .iter()
        .map(|low_point| get_basin_size(low_point, heightmap))
        .collect()
}

// Part one
fn get_low_points(heightmap: &[u32]) -> HashMap<usize, u32> {
    heightmap
        .iter()
        .enumerate()
        .filter(|&(index, height)| {
            get_adjacent_locations(index, heightmap)
                .iter()
                .all(|(_, adjacent_height)| adjacent_height > height)
        })
        .map(|(index, &height)| (index, height))
        .collect()
}

fn get_basin_size(low_point: (&usize, &u32), heightmap: &[u32]) -> usize {
    let mut indices_visited = vec![*low_point.0];

    let mut current_location = (*low_point.0, *low_point.1);

    let mut locations_to_visit: HashMap<usize, u32> =
        get_adjacent_locations(current_location.0, heightmap);

    while locations_to_visit.is_empty() {
        locations_to_visit
            .retain(|index, &mut height| !indices_visited.contains(index) && height != 9);

        if let Some(new_location) = locations_to_visit.iter().next() {
            current_location = (*new_location.0, *new_location.1);

            indices_visited.push(current_location.0);
            locations_to_visit.remove(&current_location.0);
            locations_to_visit.extend(get_adjacent_locations(current_location.0, heightmap));
        }
    }

    indices_visited.len()
}
