use std::{fs::read_to_string, io::{Write, stdout}, ops::Range};


fn main() {
    let ranges = get_ranges_from_file("input.txt");
    let mut result:i64 = 0;
    for mut r in ranges {
        result += check_range(&mut r);
    }
    println!("{}", result);
}

fn get_ranges_from_file(filename: &str) -> Vec<Range<i64>> {
    get_ranges(&read_to_string(filename).unwrap().trim_end())
}

fn get_ranges(input: &str) -> Vec<Range<i64>> {
    Vec::from_iter(input.split(',').map(|x| get_range(x)))
}

fn get_range(textform: &str) -> Range<i64> {
    let parts = Vec::from_iter(textform.split('-').map(|x| x.parse::<i64>().unwrap()));
    Range {  start: parts[0], end: parts[1] + 1 }
}

fn decompose_number(num: i64) -> Vec<i64> {
    let mut output = Vec::<i64>::new();
    let mut remains = num;
    while remains >= 10 {
        output.insert(0, remains % 10);
        remains /= 10;
    }
    output.insert(0, remains);
    output
}

fn check_vec_equality(v1: &[i64], v2: &[i64]) -> bool {
    if v1.len() != v2.len() {
        return false;
    }
    for i in 0..v1.len() {
        if v1[i] != v2[i] {
            return false;
        }
    }
    true
}

fn check_vec(data: &Vec<i64>) -> bool {
    let digit_len = data.len();
    for i in 2..(digit_len + 1) {
        if check_vec_across_divisors(data, i) {
            return true;
        }
    }
    false
}

fn check_vec_across_divisors(data: &Vec<i64>, divisor: usize) -> bool {
    let digit_len = data.len();
    if digit_len % divisor != 0 {
        return false;
    }
    let block = digit_len / divisor;
    println!("Dividing {digit_len} by {divisor} into {block}");
    let _ = stdout().flush();
    for i in 0..(divisor - 1) {
        let start = block * i;
        println!("Starting at {start}");
        let _ = stdout().flush();
        if !check_vec_equality(&data[start..(start + block)], &data[(start + block)..(start + block * 2)]) {
            return false;
        }
    }
    true
}

fn check_number(num: i64) -> bool {
    let digits = decompose_number(num);
    check_vec(&digits)
}

fn check_range(r: &mut Range<i64>) -> i64 {
    let mut result: i64 = 0;
    for i in r {
        if check_number(i) {
            result += i64::from(i);
        }
    }
    result
}