use std::ops::Range;

fn get_row_number(ticket_number: &str) -> usize {
    ticket_number[..7]
        .chars()
        .fold((0, 127), |range, c| {
            if c == 'F' {
                (range.0, mid_of(range))
            } else {
                (mid_of(range), range.1)
            }
        })
        .0
}

fn get_column_number(ticket_number: &str) -> usize {
    ticket_number[7..10]
        .chars()
        .fold((0, 7), |range, c| {
            if c == 'L' {
                (range.0, mid_of(range))
            } else {
                (mid_of(range), range.1)
            }
        })
        .0
}

fn get_seat_id(ticket_number: &str) -> usize {
    let row = get_row_number(&ticket_number);
    let column = get_column_number(&ticket_number);
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
        assert_eq!(get_row_number("FBFBBFFRLR"), 44);
    }

    #[test]
    fn get_column_number_given_ticket() {
        assert_eq!(get_column_number("FBFBBFFRLR"), 5);
    }

    #[test]
    fn get_seat_id_for_given_ticket() {
        assert_eq!(get_seat_id("FBFBBFFRLR"), 357)
    }
}
