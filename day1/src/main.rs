use std::{
    cmp::Ordering,
    fs,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

// Counts how many times the first value in a list repeats.
// It returns slice to the next part of the vector, the repeating value and the number of times it appears
fn count_first_same<T: Ord + Copy>(list: &[T]) -> (&[T], T, usize) {
    if list.is_empty() {
        panic!("List should never be empty logically");
    }
    let sim_val = &list[0];
    for (index, value) in list.iter().enumerate() {
        if value != sim_val {
            return (&list[index..], *sim_val, index);
        }
    }
    (&[], *sim_val, 0)
}

fn main() {
    let content =
        fs::read_to_string("input.txt").expect("Please provide input text file input.txt");

    let line_count = content.lines().count();
    let mut left: Vec<i32> = Vec::with_capacity(line_count);
    let mut right: Vec<i32> = Vec::with_capacity(line_count);
    // Converts input.txt to better data structures
    for line in content.lines() {
        let mut iter = line.split("   ");
        // Assume input is correct
        if let (Some(left_val), Some(right_val)) = (iter.next(), iter.next()) {
            let err_msg = "Input should be correct";
            let left_val: i32 = left_val.trim().parse().expect(err_msg);
            let right_val: i32 = right_val.trim().parse().expect(err_msg);
            left.push(left_val);
            right.push(right_val);
        }
    }

    let start = Instant::now();
    // Sorts both arrays in two threads
    let left = Arc::new(Mutex::new(left));
    let left_clone = Arc::clone(&left);
    let left_handle = thread::spawn(move || {
        left_clone.lock().expect("Failed to lock mutex").sort();
    });
    right.sort();
    if let Err(_) = left_handle.join() {
        eprintln!("Error: could not execute thread");
        std::process::exit(1);
    }

    // Mutex is now permanently unlocked
    let left = left.lock().expect("Failed to lock mutex");
    let mut total_distance = 0;
    for (left, right) in left.iter().zip(right.iter()) {
        total_distance += (left - right).abs();
    }

    let mut left = &left[..];
    let mut right = &right[..];
    let mut similarity_score = 0;
    while !left.is_empty() && !right.is_empty() {
        let (left_slice, left_val, left_nbr): (&[i32], i32, usize) = count_first_same(left);
        let (right_slice, right_val, right_nbr): (&[i32], i32, usize) = count_first_same(right);

        match left_val.cmp(&right_val) {
            Ordering::Less => left = left_slice,
            Ordering::Greater => right = right_slice,
            Ordering::Equal => {
                left = left_slice;
                right = right_slice;
                similarity_score += left_val * left_nbr as i32 * right_nbr as i32;
            }
        }
    }

    let duration = start.elapsed();
    println!("Total distance: {total_distance}\nSimilarity score: {similarity_score}\nTime elapsed: {duration:?}");
}
