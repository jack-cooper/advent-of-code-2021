fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("assets/input.txt")?;

    let fishes: Vec<i8> = input.trim().split(',').flat_map(str::parse).collect();

    dbg!(fish_after_days(80, fishes).len());

    Ok(())
}

fn fish_after_days(days: u8, mut fishes: Vec<i8>) -> Vec<i8> {
    for _ in 0..days {
        let mut births: usize = 0;

        for fish in fishes.iter_mut() {
            let aged_fish = *fish - 1;

            *fish = if aged_fish < 0 {
                births += 1;
                6
            } else {
                aged_fish
            }
        }

        fishes.append(&mut vec![8_i8; births])
    }

    fishes
}
