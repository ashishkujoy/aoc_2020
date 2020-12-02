#[derive(Debug, PartialEq, PartialOrd)]
struct PasswordPolicy<'i> {
    char: &'i str,
    min_occurance: u16,
    max_occurance: u16,
}

impl<'i> PasswordPolicy<'i> {
    fn parse(input: &'i str) -> Self {
        let mut inputs = input.split_ascii_whitespace();

        let mut min_max = inputs
            .next()
            .expect("min and max count required")
            .split("-")
            .map(|e| e.parse::<u16>().expect("min/max should be u16"));

        let min: u16 = min_max.next().expect("min occurance required");

        let max: u16 = min_max.next().expect("max occurance required");

        let char = inputs.next().unwrap();

        PasswordPolicy {
            char,
            min_occurance: min,
            max_occurance: max,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_policy_from_given_input() {
        let input = "1-7 j";
        let policy = PasswordPolicy::parse(input);

        assert_eq!(
            policy,
            PasswordPolicy {
                char: "j",
                min_occurance: 1,
                max_occurance: 7
            }
        )
    }
}
