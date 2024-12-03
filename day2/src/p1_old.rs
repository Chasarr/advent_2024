// This function is an example of letting functional one liners get in the way of readable and adaptable code
// I learned a lot by writing this!
fn p1(input: &Vec<Vec<i32>>) {
    let sliding_diff = input
        .iter()
        .map(|report| {
            report
                .windows(2)
                .map(|window| window[0] - window[1])
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let in_bounds = sliding_diff
        .into_iter()
        .filter(|report_diff| {
            report_diff
                .iter()
                .all(|&value| value.abs() <= 3 && value != 0)
        })
        .collect::<Vec<Vec<i32>>>();

    let same_sign = in_bounds
        .into_iter()
        .filter(|report_diff| {
            report_diff.iter().all(|&diff| diff.is_positive())
                || report_diff.iter().all(|&diff| diff.is_negative())
        })
        .collect::<Vec<Vec<i32>>>();
    println!("p1: {}", same_sign.len());
}
