use std::{collections::HashMap, iter};

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("assets/input.txt")?;

    dbg!(calculate_output_1478_digits(&input));
    dbg!(calculate_output_sum(&input));

    Ok(())
}

// Part one
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

// Part two
fn calculate_output_sum(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| {
            let mut entry = line.split(" | ");

            entry.next().zip(entry.next())
        })
        .map(|(input, output)| {
            let segment_mappings = get_segment_mappings(input);
            map_output_segments(output, &segment_mappings)
        })
        .map(output_to_digits)
        .sum()
}

fn output_to_digits(output: String) -> usize {
    let character_digit_mapping: HashMap<&str, &str> = HashMap::from_iter([
        ("abcefg", "0"),
        ("cf", "1"),
        ("acdeg", "2"),
        ("acdfg", "3"),
        ("bcdf", "4"),
        ("abdfg", "5"),
        ("abdefg", "6"),
        ("acf", "7"),
        ("abcdefg", "8"),
        ("abcdfg", "9"),
    ]);

    output
        .split_whitespace()
        .map(|digit| {
            let mut digit_chars: Vec<char> = digit.chars().collect();
            digit_chars.sort();
            digit_chars.iter().collect::<String>()
        })
        .flat_map(|digit| character_digit_mapping.get(&digit[..]))
        .fold(String::new(), |output_value, digit| {
            format!("{}{}", output_value, digit)
        })
        .parse()
        .expect("The character_digit_mapping HashMap only allows this to be a valid value")
}

fn get_segment_mappings(input: &str) -> HashMap<char, char> {
    let digits: Vec<&str> = input.split_whitespace().collect();

    let one = digits
        .iter()
        .find(|digit| digit.len() == 2)
        .expect("A 1 should be provided in the input!");
    let seven = digits
        .iter()
        .find(|digit| digit.len() == 3)
        .expect("A 7 should be provided in the input!");

    let segment_a_char = get_segment_a_char(one, seven);

    let four = digits
        .iter()
        .find(|digit| digit.len() == 4)
        .expect("A 4 should be provided in the input!");

    let (segment_g_char, nine) = get_segment_g_char_and_nine_digit(four, segment_a_char, &digits);

    let eight = digits
        .iter()
        .find(|digit| digit.len() == 7)
        .expect("An 8 should be provided in the input!");

    let segment_e_char = get_segment_e_char(eight, nine);

    let segment_d_char = get_segment_d_char(segment_a_char, segment_g_char, one, &digits);

    let segment_b_char = get_segment_b_char(one, four, segment_d_char);

    let segment_c_char = get_segment_c_char(
        [
            segment_a_char,
            segment_d_char,
            segment_e_char,
            segment_g_char,
        ],
        &digits,
    );

    let segment_f_char = get_segment_f_char(
        [
            segment_a_char,
            segment_b_char,
            segment_d_char,
            segment_g_char,
        ],
        &digits,
    );

    HashMap::from_iter([
        (segment_a_char, 'a'),
        (segment_b_char, 'b'),
        (segment_c_char, 'c'),
        (segment_d_char, 'd'),
        (segment_e_char, 'e'),
        (segment_f_char, 'f'),
        (segment_g_char, 'g'),
    ])
}

// ### Phase I: Determining Segment A
fn get_segment_a_char(one: &str, seven: &str) -> char {
    seven
        .chars()
        .find(|&segment| !one.contains(segment))
        .expect("A 7 should be a 1 with an extra segment")
}

// ### Phase IV-A: Determining Segment B
fn get_segment_b_char(one: &str, four: &str, segment_d_char: char) -> char {
    let pseudo_digit: String = one.chars().chain(iter::once(segment_d_char)).collect();

    four.chars()
        .find(|&segment| !pseudo_digit.contains(segment))
        .expect("A 4 is built up of our pseudo-digit + segment B!")
}

// ### Phase IV-B: Determining Segment C
fn get_segment_c_char(adeg_chars: [char; 4], digits: &[&str]) -> char {
    let two = digits
        .iter()
        .find(|digit| digit.len() == 5 && adeg_chars.iter().all(|&segment| digit.contains(segment)))
        .expect("A 2 is built up of ADEG + segment C!");

    two.chars()
        .find(|segment| !adeg_chars.contains(segment))
        .expect("A 2 is built up of ADEG + segment C!")
}

// ### Phase III-B: Determining Segment D
fn get_segment_d_char(
    segment_a_char: char,
    segment_g_char: char,
    one: &str,
    digits: &[&str],
) -> char {
    let pseudo_digit: String = one
        .chars()
        .chain([segment_a_char, segment_g_char])
        .collect();

    let three = digits
        .iter()
        .find(|digit| {
            digit.len() == 5 && pseudo_digit.chars().all(|segment| digit.contains(segment))
        })
        .expect("A 3 is built up of our pseudo-digit + segment D!");

    three
        .chars()
        .find(|&segment| !pseudo_digit.contains(segment))
        .expect("A 3 is built up of our pseudo-digit + segment D!")
}

// ### Phase III-A: Determining Segment E
fn get_segment_e_char(eight: &str, nine: &str) -> char {
    eight
        .chars()
        .find(|&segment| !nine.contains(segment))
        .expect("An 8 is built up of a 9 + segment E!")
}

// ### Phase IV-C: Determining Segment F
fn get_segment_f_char(abdg_chars: [char; 4], digits: &[&str]) -> char {
    let five = digits
        .iter()
        .find(|digit| digit.len() == 5 && abdg_chars.iter().all(|&segment| digit.contains(segment)))
        .expect("A 5 is built up of ABDG + segment F!");

    five.chars()
        .find(|segment| !abdg_chars.contains(segment))
        .expect("A 5 is built up of ABDG + segment F!")
}

// ### Phase II: Determining Segment G
fn get_segment_g_char_and_nine_digit<'a>(
    four: &str,
    segment_a_character: char,
    digits: &'a [&str],
) -> (char, &'a str) {
    let pseudo_digit: String = four
        .chars()
        .chain(iter::once(segment_a_character))
        .collect();

    let nine = digits
        .iter()
        .find(|digit| {
            digit.len() == 6 && pseudo_digit.chars().all(|segment| digit.contains(segment))
        })
        .expect("A 9 is built up of our pseudo-digit + segment G!");

    let segment_g_char = nine
        .chars()
        .find(|&segment| !pseudo_digit.contains(segment))
        .expect("A 9 is built up of our pseudo-digit + segment G!");

    (segment_g_char, nine)
}

fn map_output_segments(output: &str, segment_mappings: &HashMap<char, char>) -> String {
    output
        .chars()
        .flat_map(|segment| segment_mappings.get(&segment).or(Some(&' ')))
        .collect()
}
