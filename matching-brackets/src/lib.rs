

const SQUARE_OPEN: char = '[';
const SQUARE_CLOSE: char = ']';
const CURLY_OPEN: char = '{';
const CURLY_CLOSE: char = '}';
const ROUND_OPEN: char = '(';
const ROUND_CLOSE: char = ')';

pub fn is_correct_closing_brace(open: char, close: char) -> bool {
    if open == SQUARE_OPEN && close == SQUARE_CLOSE {
        return true;
    }
    if open == CURLY_OPEN && close == CURLY_CLOSE {
        return true;
    }
    if open == ROUND_OPEN && close == ROUND_CLOSE {
        return true;
    }
    false
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut open_brace_stack = String::from("");

    for current_char in string.chars() {
        match current_char {
            SQUARE_OPEN | CURLY_OPEN | ROUND_OPEN => {
                open_brace_stack.push(current_char);
            },
            SQUARE_CLOSE | CURLY_CLOSE | ROUND_CLOSE => {
                let last_open_brace = open_brace_stack.pop();
                match last_open_brace {
                    Some(open_brace) => {
                        if !is_correct_closing_brace(open_brace, current_char) {
                            return false;
                        }
                    },
                    None => {
                        return false;
                    }
                }

            },
            _ => continue
        }
    }
    if open_brace_stack.len() == 0 {
        return true;
    }
    false
}
