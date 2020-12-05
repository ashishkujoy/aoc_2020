use regex::Regex;
use std::collections::HashMap;

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

fn count_valid_password_docs_v2(docs: &str) -> usize {
    docs.split("\n\n")
        .filter(|p| PasswordDoc::parse(p).map(|e| e.is_valid()) == Some(true))
        .count()
}
#[derive(Debug, PartialOrd, PartialEq)]
struct Year {
    value: usize,
}

impl Year {
    fn parse(input: &str) -> Option<Self> {
        input.parse::<usize>().ok().map(|value| Year { value })
    }

    fn is_between_include_edges(&self, min: usize, max: usize) -> bool {
        (min..max).contains(&self.value)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Height<'i> {
    value: usize,
    unit: &'i str,
}

impl<'i> Height<'i> {
    fn parse(input: &'i str) -> Option<Self> {
        if input.len() < 3 {
            return None;
        }
        if !input.ends_with("cm") && !input.ends_with("in") {
            return None;
        }
        match input[..input.len() - 2].parse::<usize>() {
            Ok(value) => Some(Height {
                value,
                unit: &input[input.len() - 2..input.len()],
            }),
            Err(_) => None,
        }
    }

    fn is_valid(&self) -> bool {
        let expected_range = if self.unit == "cm" { 150..194 } else { 59..77 };
        expected_range.contains(&self.value)
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Color<'i> {
    value: &'i str,
}

impl<'i> Color<'i> {
    fn is_valid_hair_color(&self) -> bool {
        let regex = Regex::new(r"(^#[(0-9)(a-f)]{6})").unwrap();
        self.value.len() == 7 && regex.is_match(self.value)
    }

    fn is_valid_eye_color(&self) -> bool {
        vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&self.value)
    }
}
#[derive(Debug, PartialOrd, PartialEq)]
struct PassportId<'i> {
    value: &'i str,
}

impl<'i> PassportId<'i> {
    fn is_valid(&self) -> bool {
        let regex = Regex::new(r"([0-9]{9})").unwrap();
        self.value.len() == 9 && regex.is_match(self.value)
    }
}
#[derive(Debug, PartialOrd, PartialEq)]
struct PasswordDoc<'i> {
    birth_year: Year,
    issue_year: Year,
    expiration_year: Year,
    height: Height<'i>,
    hair_color: Color<'i>,
    eye_color: Color<'i>,
    password_id: PassportId<'i>,
}

impl<'i> PasswordDoc<'i> {
    fn parse(input: &'i str) -> Option<Self> {
        let mut tokens = input.split_ascii_whitespace();
        let mut hash_map = HashMap::new();
        tokens.for_each(|token| {
            let mut t = token.split(":");
            let token_name = t.next().expect("Token name expected");
            let token_value = t.next().expect("Token value expected");
            hash_map.insert(token_name, token_value);
        });

        let birth_year = hash_map.get("byr").map(|e| Year::parse(e)).flatten()?;
        let issue_year = hash_map.get("iyr").map(|e| Year::parse(e)).flatten()?;
        let expiration_year = hash_map.get("eyr").map(|e| Year::parse(e)).flatten()?;
        let height = hash_map.get("hgt").map(|e| Height::parse(e)).flatten()?;
        let hair_color = hash_map.get("hcl").map(|e| Color { value: e })?;
        let eye_color = hash_map.get("ecl").map(|e| Color { value: e })?;
        let password_id = hash_map.get("pid").map(|e| PassportId { value: e })?;

        Some(PasswordDoc {
            birth_year,
            issue_year,
            expiration_year,
            height,
            eye_color,
            password_id,
            hair_color,
        })
    }

    fn is_valid(&self) -> bool {
        self.birth_year.is_between_include_edges(1920, 2003)
            && self.issue_year.is_between_include_edges(2010, 2021)
            && self.expiration_year.is_between_include_edges(2020, 2031)
            && self.height.is_valid()
            && self.eye_color.is_valid_eye_color()
            && self.hair_color.is_valid_hair_color()
            && self.password_id.is_valid()
    }
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
        assert_eq!(
            Height::parse("59in").unwrap(),
            Height {
                value: 59,
                unit: "in"
            }
        );
        assert_eq!(
            Height::parse("59cm").unwrap(),
            Height {
                value: 59,
                unit: "cm"
            }
        );
    }
    #[test]
    fn give_non_for_invalid_heights() {
        assert!(Height::parse("59").is_none());
        assert!(Height::parse("59c").is_none());
        assert!(Height::parse("cm").is_none());
        assert!(Height::parse("5e9cm").is_none());
    }

    #[test]
    fn check_a_hair_color_is_valid() {
        assert!(Color { value: "#123653" }.is_valid_hair_color());
        assert!(Color { value: "#abcdef" }.is_valid_hair_color());
        assert!(Color { value: "#123abc" }.is_valid_hair_color());
    }
    #[test]
    fn check_a_hair_color_not_starting_with_hash_followed_by_either_6_digit_or_small_case_alphabet_from_a_to_f(
    ) {
        assert!(!Color { value: "#123abz" }.is_valid_hair_color());
        assert!(!Color { value: "#1234567" }.is_valid_hair_color());
        assert!(!Color { value: "1234567" }.is_valid_hair_color());
        assert!(!Color { value: "abcdef" }.is_valid_hair_color());
    }

    #[test]
    fn a_nine_digit_number_should_be_valid_password_id() {
        assert!(PassportId { value: "000012349" }.is_valid());
        assert!(PassportId { value: "123456789" }.is_valid());
    }

    #[test]
    fn a_non_nine_digit_number_should_not_be_valid_password_id() {
        assert!(!PassportId { value: "123" }.is_valid());
        assert!(!PassportId {
            value: "1234567891"
        }
        .is_valid());
        assert!(!PassportId { value: "1234567ab" }.is_valid());
    }

    #[test]
    fn parse_proper_formatted_document() {
        let password_doc = PasswordDoc::parse(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f",
        )
        .unwrap();

        assert!(password_doc.is_valid());
        assert!(PasswordDoc::parse(
            "eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"
        )
        .unwrap()
        .is_valid());
    }

    #[test]
    fn give_non_for_improper_documents() {
        assert_ne!(
            PasswordDoc::parse(
                "eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"
            )
            .map(|p| p.is_valid()),
            Some(true)
        );
        assert_eq!(
            PasswordDoc::parse(
                "iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946"
            )
            .map(|p| p.is_valid()),
            Some(false)
        );
        assert_eq!(
            PasswordDoc::parse(
                "hcl:dab227 iyr:2012
                ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"
            )
            .map(|p| p.is_valid()),
            Some(false)
        );
        assert_eq!(
            PasswordDoc::parse(
                "hgt:59cm ecl:zzz
                eyr:2038 hcl:74454a iyr:2023
                pid:3556412378 byr:2007"
            )
            .map(|p| p.is_valid()),
            Some(false)
        );
    }
}
