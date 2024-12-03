fn prepare_data() -> Vec<Vec<i32>> {
    let input = include_str!("../input.txt");
    input
        .lines()
        .map(|line| {
            let str_nbrs = line.split(" ");
            str_nbrs
                .map(|value| value.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

// Counts safe reports with at most one error in it
fn count_safe_forgiving(input: &Vec<Vec<i32>>) -> u32 {
    let mut num_safe: u32 = 0;
    for report in input {
        if let Some(index) = error_at(&report) {
            let mut remove_left = report.clone();
            let mut remove_right = report.clone();
            remove_left.remove(index);
            remove_right.remove(index + 1);
            if let (Some(_), Some(_)) = (error_at(&remove_left), error_at(&remove_right)) {
                continue;
            }
        }
        num_safe += 1;
    }
    num_safe
}

// Finds the first faulty value if it exists.
// Some(index): index and index + 1 can be faulty and need to be removed and checked
fn error_at(report: &Vec<i32>) -> Option<usize> {
    let diff = report
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<i32>>();

    let first_zero_or_big = diff.iter().position(|&x| x == 0 || x.abs() > 3);
    if first_zero_or_big.is_some() {
        return first_zero_or_big;
    }

    let num_rising = diff.iter().filter(|x| x.is_positive()).count();
    let num_falling = diff.iter().filter(|x| x.is_negative()).count();
    if num_rising > num_falling {
        // Only positive values allowed
        let first_neg = diff.iter().position(|&x| x.is_negative());
        if first_neg.is_some() {
            return first_neg;
        }
    } else {
        let first_pos = diff.iter().position(|&x| x.is_positive());
        if first_pos.is_some() {
            return first_pos;
        }
    }
    None
}

fn main() {
    let input = prepare_data();
    let p1 = input
        .iter()
        .filter(|report| error_at(&report).is_none())
        .count();
    let p2 = count_safe_forgiving(&input);

    println!("p1: {p1}");
    println!("p2: {p2}");
}
