use std::iter::zip;

use aoclib::{input, output};

fn main() {
    let input = input("input");
    let mut vec_left = Vec::new();
    let mut vec_right = Vec::new();
    // Data preprocessing code goes here
    for line in input.lines() {
        let (left, right) = line.split_once("   ").unwrap();
        vec_left.push(left.parse::<u32>().unwrap());
        vec_right.push(right.parse::<u32>().unwrap());
        vec_left.sort();
        vec_right.sort();
    }

    p1(&vec_left, &vec_right);
    p2(&vec_left, &vec_right);
}

#[inline]
fn p1(vec_left: &Vec<u32>, vec_right: &Vec<u32>) {
    let iter = zip(vec_left, vec_right);
    let result: u32 = iter.map(|(a, b)| a.abs_diff(*b)).sum();
    output(result)
}

#[inline]

fn p2(vec_left: &[u32], vec_right: &[u32]) {
    let mut sum = 0u32;
    for n in vec_left.iter() {
        let mut count = 0u32;
        for i in vec_right.iter() {
            // print!("{i}");
            if *n == *i {
                count += 1;
            }
        }
        sum += count * n;
    }
    output(sum);
}
