fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("assets/input.txt")?;

    dbg!(calculate_output_1478_digits(&input));

    Ok(())
}

fn calculate_output_1478_digits(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| {
            let mut entry = line.split(" | ");

            entry.next();
            entry.next()
        })
        .flat_map(|output_values| output_values.split(' '))
        .filter(|output_value| {
            let number_of_signals = output_value.len();

            number_of_signals == 2
                || number_of_signals == 3
                || number_of_signals == 4
                || number_of_signals == 7
        })
        .count()
}
