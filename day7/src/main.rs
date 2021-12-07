fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("assets/input.txt")?;

    let positions: Vec<i32> = input.trim().split(',').flat_map(str::parse).collect();

    let (minimum_position, maximum_position) =
        positions
            .iter()
            .fold((i32::MAX, i32::MIN), |min_max_positions, position| {
                (
                    (*position).min(min_max_positions.0),
                    (*position).max(min_max_positions.1),
                )
            });

    if let Some(minimum_distance) = (minimum_position..=maximum_position)
        .map(|position| calculate_cumulative_distance_to_target(position, &positions))
        .min()
    {
        dbg!(minimum_distance);
    }

    Ok(())
}

fn calculate_cumulative_distance_to_target(target: i32, positions: &[i32]) -> i32 {
    positions
        .iter()
        .map(|position| {
            let distance = (position - target).abs();
            distance * (distance + 1) / 2
        })
        .sum()
}
