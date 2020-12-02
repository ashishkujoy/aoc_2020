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

    fn is_adhered_by(&self, password: &str) -> bool {
        let char_occurance_count = password.split("").filter(|e| e == &self.char).count();
        (self.min_occurance..self.max_occurance + 1).contains(&(char_occurance_count as u16))
    }
}

fn parse_password_and_policy<'i>(input: &'i str) -> (&str, PasswordPolicy<'i>) {
    let mut inputs = input.split(": ");
    let password_policy = PasswordPolicy::parse(inputs.next().unwrap());
    (inputs.next().expect("missing password"), password_policy)
}

fn get_count_of_invalid_passwords(inputs: &Vec<&str>) -> usize {
    inputs
        .into_iter()
        .filter(|input| {
            let (password, password_policy) = parse_password_and_policy(input);
            !password_policy.is_adhered_by(password)
        })
        .count()
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

    #[test]
    fn pass_password_and_password_policy() {
        let input = "1-7 j: vrfjljjwbsv";
        let (password, password_policy) = parse_password_and_policy(input);

        assert_eq!(password, "vrfjljjwbsv");
        assert_eq!(
            password_policy,
            PasswordPolicy {
                char: "j",
                min_occurance: 1,
                max_occurance: 7
            }
        );
    }

    #[test]
    fn check_a_password_having_given_char_between_min_and_max_count_adhere_to_password_policy() {
        let password_policy = PasswordPolicy {
            char: "j",
            min_occurance: 1,
            max_occurance: 7,
        };

        assert!(password_policy.is_adhered_by("vrfjljjwbsv"))
    }
    #[test]
    fn check_a_password_having_given_char_between_below_min_count_does_not_adhere_to_password_policy(
    ) {
        let password_policy = PasswordPolicy {
            char: "j",
            min_occurance: 5,
            max_occurance: 7,
        };

        assert!(!password_policy.is_adhered_by("vrfjljjwbsv"))
    }
    #[test]
    fn check_a_password_having_given_char_between_above_max_count_does_not_adhere_to_password_policy(
    ) {
        let password_policy = PasswordPolicy {
            char: "j",
            min_occurance: 1,
            max_occurance: 2,
        };

        assert!(!password_policy.is_adhered_by("vrfjljjwbsv"))
    }

    #[test]
    fn give_count_of_password_not_adhering_to_policy() {
        let inputs = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        assert_eq!(get_count_of_invalid_passwords(&inputs), 1);
    }
}
