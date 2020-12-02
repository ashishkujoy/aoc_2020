fn pair_having_sum_n(n: i64, inputs: &Vec<i64>) -> Option<Vec<i64>> {
    inputs.into_iter().find_map(|x| {
        let remaining = n - x;
        if inputs.contains(&remaining) {
            Some(vec![*x, remaining])
        } else {
            None
        }
    })
}

fn find_tuple_of_n_having_sum(size_of_tuple: u64, sum: i64, inputs: &Vec<i64>) -> Option<Vec<i64>> {
    if size_of_tuple <= 2 {
        pair_having_sum_n(sum, inputs)
    } else {
        let mut current_index = 0;
        inputs.into_iter().find_map(|number| {
            current_index += 1;
            find_tuple_of_n_having_sum(
                size_of_tuple - 1,
                sum - number,
                &inputs[current_index..inputs.len()].to_vec(),
            )
            .map(|mut r| {
                r.insert(0, *number);
                r
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_pair_having_sum_2020() {
        let inputs = vec![1721, 979, 366, 299, 675, 1456];
        let pair = pair_having_sum_n(2020, &inputs);

        assert_eq!(pair, Some(vec![1721, 299]))
    }
    #[test]
    fn find_tuple_of_size_3_having_sum_2020() {
        let inputs = vec![1000, 979, 420, 299, 600, 1456];
        let pair = find_tuple_of_n_having_sum(3, 2020, &inputs);

        assert_eq!(pair, Some(vec![1000, 420, 600]))
    }
}
