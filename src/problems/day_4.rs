use std::num::ParseIntError;

fn is_valid_password_doc(doc: &str) -> bool {
    vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .all(|e| doc.contains(e))
}

fn count_valid_password_docs(docs: &str) -> usize {
    docs.split("\n\n")
        .filter(|doc| is_valid_password_doc(doc))
        .count()
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Height<'i> {
    value: usize,
    unit: &'i str,
}

impl<'i> Height<'i> {
    fn parse(input: &'i str) -> Option<Height> {
        if input.len() < 3 {
            return None;
        }
        if !input.ends_with("cm") && !input.ends_with("in") {
            return None;
        }
        match input[..input.len() - 2].parse::<usize>() {
            Ok(value) => Some(Height { value, unit: &input[input.len() - 2..input.len()] }),
            Err(_) => None
        }
    }
}

struct PasswordDoc<'i> {
    birth_year: usize,
    issue_year: usize,
    expiration_year: usize,
    height: Height<'i>,
    hair_color: &'i str,
    eye_color: &'i str,
    password_id: &'i str,
    country_id: &'i str,
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_doc_is_valid() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
                            byr:1937 iyr:2017 cid:147 hgt:183cm";

        let is_valid = is_valid_password_doc(input);

        assert!(is_valid);
    }

    #[test]
    fn check_doc_is_invalid() {
        let input = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
                            hcl:#cfa07d byr:1929";
        assert!(!is_valid_password_doc(input))
    }

    #[test]
    fn could_valid_password_docs_in_given_batch() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
                            byr:1937 iyr:2017 cid:147 hgt:183cm

                            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
                            hcl:#cfa07d byr:1929

                            hcl:#ae17e1 iyr:2013
                            eyr:2024
                            ecl:brn pid:760753108 byr:1931
                            hgt:179cm

                            hcl:#cfa07d eyr:2025 pid:166559648
                            iyr:2011 ecl:brn hgt:59in";

        assert_eq!(count_valid_password_docs(input), 2);
    }

    #[test]
    fn parse_height_when_given_input_is_a_number_followed_by_cm_or_in() {
        assert_eq!(Height::parse("59in").unwrap(), Height { value: 59, unit: "in" });
        assert_eq!(Height::parse("59cm").unwrap(), Height { value: 59, unit: "cm" });
    }
    #[test]
    fn give_non_for_invalid_heights() {
        assert!(Height::parse("59").is_none());
        assert!(Height::parse("59c").is_none());
        assert!(Height::parse("cm").is_none());
        assert!(Height::parse("5e9cm").is_none());
    }
}
