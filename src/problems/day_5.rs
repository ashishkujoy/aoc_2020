use std::ops::Range;

fn get_get_id_for_code(code: &str, lower_bond: usize, upper_bond: usize) -> usize {
    code.chars()
        .fold((lower_bond, upper_bond), |range, c| {
            if c == 'F' || c == 'L' {
                (range.0, mid_of(range))
            } else {
                (mid_of(range), range.1)
            }
        })
        .0
}

fn get_seat_id(ticket_number: &str) -> usize {
    let row = get_get_id_for_code(&ticket_number[..7], 0, 127);
    let column = get_get_id_for_code(&ticket_number[7..10], 0, 7);
    (row * 8) + column
}

fn mid_of(tuple: (usize, usize)) -> usize {
    (tuple.0 + tuple.1 + 1) / 2
}

fn get_largest_seat_id(tickets: &str) -> usize {
    tickets
        .split_ascii_whitespace()
        .map(|ticket| get_seat_id(ticket))
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_row_number_given_ticket() {
        assert_eq!(get_get_id_for_code("FBFBBFF", 0, 127), 44);
    }

    #[test]
    fn get_column_number_given_ticket() {
        assert_eq!(get_get_id_for_code("RLR", 0, 7), 5);
    }

    #[test]
    fn get_seat_id_for_given_ticket() {
        assert_eq!(get_seat_id("FBFBBFFRLR"), 357)
    }
}
