use std::str::Chars;

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("assets/input.txt")?;

    let illegal_brackets = get_illegal_brackets(&input);

    let syntax_error_score: u64 = illegal_brackets.map(get_bracket_score).sum();

    dbg!(syntax_error_score);

    Ok(())
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

fn get_closing_bracket(opening_bracket: char) -> char {
    match opening_bracket {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("That wasn't a bracket!"),
    }
}

fn get_first_illegal_character(chars: Chars) -> Option<char> {
    let mut opening_bracket_stack: Vec<char> = Vec::new();

    for char in chars {
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
fn get_illegal_brackets<'a>(input: &'a str) -> impl Iterator<Item = char> + 'a {
    input
        .lines()
        .map(str::chars)
        .flat_map(get_first_illegal_character)
}

fn is_opening_bracket(char: char) -> bool {
    match char {
        '(' | '[' | '{' | '<' => true,
        _ => false,
    }
}
