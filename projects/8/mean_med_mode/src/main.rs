/*
Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
*/

use std::collections::HashMap;
fn main() {
    println!("Hello, world!");

    let mut v = vec![
        22, 23, 24, 24, 25, 9, 10, 12, 13, 13, 13, 15, 15, 16, 16, 18,
    ];

    println!("v: {:?}", v);

    let med = median(&mut v);
    let mean = mean(&v);
    let mode = mode(&v);

    println!("Median is {med}");
    println!("Mean is {mean}");
    println!("Mode is {mode}");

    // Notice it is sorted now
    println!("v: {:?}", v);
}

fn mean(v: &[u32]) -> f32 {
    v.iter().sum::<u32>() as f32 / v.len() as f32
}

fn median(v: &mut [u32]) -> f32 {
    v.sort();
    let mid = v.len() / 2;

    if (v.len() % 2) == 0 {
        mean(&[v[mid - 1], v[mid]])
    } else {
        v[mid] as f32
    }
}

fn mode(v: &Vec<u32>) -> u32 {
    let mut counts = HashMap::new();

    for num in v {
        let count = counts.entry(num).or_insert(0);
        *count += 1
    }

    // Reference the hashmap
    *counts
        .into_iter() // Turn it into an iterable
        .max_by_key(|&(_, count)| count) // Find max max based on each key, looking at the count
        .map(|(key, _)| key) // Get the key that has the max
        .expect("Cannot find the mode of 0 numbers")
}
