use std::fs;

/// Day 1 -> 1st problem
pub fn get_total_difference() -> i32 {
    let file_read = fs::read_to_string("d1_1_input.txt").expect("Couldn't read file");
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    let mut total_sum = 0;

    for text in file_read.lines() {
        let line: Vec<&str> = text.split("   ").collect();
        let left_side: i32 = line[0].parse().expect("Couldn't convert to i32");
        let right_side: i32 = line[1].parse().expect("Couldn't convert to i32");

        left_list.push(left_side);
        right_list.push(right_side);
    }
    left_list.sort();
    right_list.sort();

    // Both have the same length of index so we can take one as a common for both
    let total_index = left_list.len();
    for index in 0..total_index {
        total_sum += (left_list[index] - right_list[index]).abs();
    }

    total_sum
}

/// Day 1 -> 2nd problem
pub fn get_product_difference_sum() -> i32 {
    let file_read = fs::read_to_string("d1_2_input.txt").expect("Couldn't read file");
    let mut total_sum = 0;
    let mut left_list_ = Vec::new();
    let mut right_list = Vec::new();

    for text in file_read.lines() {
        let line: Vec<&str> = text.split("   ").collect();
        let left_side: i32 = line[0].parse().expect("Couldn't convert to i32");
        let right_side: i32 = line[1].parse().expect("Couldn't convet to i32");

        left_list_.push(left_side);
        right_list.push(right_side);
    }
    for l in left_list_ {
        let count = right_list.iter().filter(|&&x| x == l).count();
        let count_int: i32 = count as i32;
        println!("Left Element: {} | Commond Found: {}", l, count_int);
        let to_sum = l * count_int;
        println!("Product Common: {}", to_sum);
        total_sum += to_sum;
    }

    total_sum
}
