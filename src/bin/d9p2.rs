


fn generate_next_row(previous: &Vec<i64>) -> Vec<i64> {
    let mut current = vec![];
    for n in 0 .. previous.len() - 1 {
        let a = previous.get(n).unwrap();
        let b = previous.get(n + 1).unwrap();
        current.push(b - a);
    }
    current
}

fn generate_tree(hist: &str) -> (Vec<Vec<i64>>, usize) {
    let mut rows = vec![];

    let first_row: Vec<i64> = hist.split_whitespace().map(|val| val.parse::<i64>().unwrap()).collect();
    println!("{:?}", first_row);
    let first_lenght = first_row.len();
    rows.push(first_row);

    loop {
        let row = generate_next_row(rows.last().unwrap());
        println!("{:?}", row);

        if row.iter().all(|x| *x == 0) {
            break;
        }

        rows.push(row);
    }

    (rows, first_lenght)
}

fn calculate_prev_for_row(tree: &Vec<Vec<i64>>, row_num: usize) -> i64 {
    if let Some(current_row) = tree.get(row_num) {
        let a = current_row.first().unwrap();
        let b = calculate_prev_for_row(tree, row_num + 1);
        a - b
    } else {
        0
    }
}

fn main() {
    
    let input = include_str!("./inputs/d9-input.txt");

    let mut result = 0;
    
    for (_, line) in input.lines().enumerate() {
        let (tree, _) = generate_tree(line);
        let next_value = calculate_prev_for_row(&tree, 0);
        
        result += next_value;
    }

    println!("Answer: {}", result);

}
