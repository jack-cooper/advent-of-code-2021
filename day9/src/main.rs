const LINE_LENGTH: usize = 100;

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("assets/input.txt")?;

    let heightmap: Vec<u32> = input
        .lines()
        .flat_map(str::chars)
        .flat_map(|digit| digit.to_digit(10))
        .collect();

    let low_points = get_low_points(&heightmap);

    let total_risk_level = low_points
        .iter()
        .fold(0, |total_risk_level, height| total_risk_level + height + 1);

    dbg!(total_risk_level);

    Ok(())
}

fn get_adjacent_heights(index: usize, heightmap: &[u32]) -> Vec<u32> {
    let mut adjacent_indices = Vec::new();

    if index % LINE_LENGTH != 99 {
        adjacent_indices.push(heightmap[index + 1]);
    }
    if index % LINE_LENGTH != 0 {
        adjacent_indices.push(heightmap[index - 1]);
    }
    if index > 99 {
        adjacent_indices.push(heightmap[index - 100]);
    }
    if index < heightmap.len() - 100 {
        adjacent_indices.push(heightmap[index + 100]);
    }

    adjacent_indices
}

// Part one
fn get_low_points(heightmap: &[u32]) -> Vec<u32> {
    heightmap
        .iter()
        .enumerate()
        .filter(|&(index, height)| {
            get_adjacent_heights(index, &heightmap)
                .iter()
                .all(|adjacent_height| adjacent_height > height)
        })
        .map(|(_, &height)| height)
        .collect()
}
