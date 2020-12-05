pub fn find_tree_on_slop(input: &Vec<Vec<u8>>, down_by: usize, shift_right_by: usize) -> usize {
    let mut result: Vec<u8> = Vec::with_capacity(input.len());

    let mut cursor = 0;
    for row_index in (0..input.len()).step_by(down_by) {
        let row = &input[row_index];
        let index = cursor % row.len();
        result.push(row[index]);
        cursor += shift_right_by;
    }
    result.into_iter().filter(|x| *x == 1).count()
}

pub fn find_product_of_trees(input: &Vec<Vec<u8>>) -> usize {
    let mut prod = 1;
    for pair in vec![[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]] {
        prod = prod * find_tree_on_slop(&input, pair[1], pair[0]);
    }
    prod
}
