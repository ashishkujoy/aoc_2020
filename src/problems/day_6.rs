use std::collections::{HashMap, HashSet};

fn count_unique_questions(questions: &str) -> usize {
    let set: HashSet<char> = questions
        .split_ascii_whitespace()
        .map(|e| e.chars())
        .flatten()
        .collect();
    set.len()
}

fn sum_of_of_unique_questions_per_group(questions_per_group: &str) -> usize {
    questions_per_group
        .split("\n\n")
        .map(|question| count_unique_questions(question))
        .sum()
}

fn foo(questions: &str) -> usize {
    let mut questions = questions.split_ascii_whitespace();
    let first_set = questions.next().unwrap();
    let remaining: Vec<&str> = questions.collect();

    first_set
        .chars()
        .filter(|c| all_contains(&remaining, c))
        .count()
}

fn all_contains(vec: &Vec<&str>, character: &char) -> bool {
    vec.into_iter().all(|v| v.contains(*character))
}

fn sum_of_questions_to_which_every_one_answered_yes_in_group(question_per_group: &str) -> usize {
    question_per_group
        .split("\n\n")
        .map(|question| foo(question))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_sum_of_unique_questions_for_given_group() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(sum_of_of_unique_questions_per_group(input), 11);
    }

    #[test]
    fn count_sum_of_question_to_which_every_one_answered_yes_in_a_group() {}

}
