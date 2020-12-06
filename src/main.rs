fn main() {
    let input = "ab
    ac";
    println!("{:?}", foo(input))
}

fn foo(questions: &str) -> Vec<char> {
    let mut questions = questions.split_ascii_whitespace();
    let first_set = questions.next().unwrap();
    let remaining: Vec<&str> = questions.collect();

    first_set
        .clone()
        .chars()
        .filter(|c| all_contains(&remaining, c))
        .collect()
}

fn all_contains(vec: &Vec<&str>, character: &char) -> bool {
    vec.into_iter().all(|v| v.contains(*character))
}
