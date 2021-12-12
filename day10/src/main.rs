use std::str::Chars;

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("assets/input.txt")?;

    let illegal_brackets = get_illegal_brackets(&input);

    let syntax_error_score: u64 = illegal_brackets.map(get_bracket_score).sum();

    dbg!(syntax_error_score);

    let incomplete_lines = get_incomplete_lines(&input);

    let mut autocomplete_scores: Vec<u64> = incomplete_lines
        .map(get_characters_to_complete_line)
        .map(|chars| {
            chars.fold(0, |score, bracket| {
                score * 5 + get_autocomplete_score(bracket)
            })
        })
        .collect();

    autocomplete_scores.sort_unstable();

    dbg!(autocomplete_scores[autocomplete_scores.len() / 2]);

    Ok(())
}

fn get_autocomplete_score(bracket: char) -> u64 {
    match bracket {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("That wasn't a closing bracket!"),
    }
}

fn get_bracket_score(bracket: char) -> u64 {
    match bracket {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("That wasn't a closing bracket!"),
    }
}

fn get_characters_to_complete_line(chars: Chars) -> impl Iterator<Item = char> {
    let mut opening_bracket_stack: Vec<char> = Vec::new();

    for char in chars {
        if is_opening_bracket(char) {
            opening_bracket_stack.push(char)
        } else {
            opening_bracket_stack.pop();
        }
    }

    opening_bracket_stack
        .into_iter()
        .map(get_closing_bracket)
        .rev()
}

fn get_closing_bracket(opening_bracket: char) -> char {
    match opening_bracket {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("That wasn't a bracket!"),
    }
}

fn get_first_illegal_character(chars: &Chars) -> Option<char> {
    let mut opening_bracket_stack: Vec<char> = Vec::new();

    for char in chars.to_owned() {
        if is_opening_bracket(char) {
            opening_bracket_stack.push(char)
        } else {
            match opening_bracket_stack.pop() {
                Some(opening_bracket) => {
                    if get_closing_bracket(opening_bracket) != char {
                        return Some(char);
                    }
                }
                None => return Some(char),
            }
        }
    }

    None
}

// Part one
fn get_illegal_brackets(input: &str) -> impl Iterator<Item = char> + '_ {
    input
        .lines()
        .map(str::chars)
        .flat_map(|chars| get_first_illegal_character(&chars))
}

fn get_incomplete_lines(input: &str) -> impl Iterator<Item = Chars> {
    input
        .lines()
        .map(str::chars)
        .filter(|chars| get_first_illegal_character(chars) == None)
}

fn is_opening_bracket(char: char) -> bool {
    matches!(char, '(' | '[' | '{' | '<')
}
