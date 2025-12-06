use std::ops::Range;

fn main() {
    let file_contents = std::fs::read_to_string("input.txt").unwrap();
    let all_lines = Vec::from_iter(file_contents.lines());
    let split_val = all_lines.iter().position(|x| *x == "").unwrap();
    let range_lines = &all_lines[..split_val];
    let item_lines = &all_lines[(split_val + 1)..];
    let mut ranges = parse_ranges(range_lines);
    let items = parse_items(item_lines);
    let mut part1_counter = 0;
    for item in items {
        if in_ranges_inclusive(item, &ranges) {
            part1_counter += 1;
        }
    }
    let mut range_set = Vec::<Range<i64>>::new();
    ranges.sort_by(|r1, r2| r1.start.cmp(&r2.start));
    for r in ranges {
        ranges_check_and_add(r, &mut range_set);
    }
    let part2_result = ranges_sum(&range_set);
    println!("{}", part1_counter);
    println!("{}", part2_result);
}

fn ranges_sum(ranges: &Vec<Range<i64>>) -> i64 {
    let mut total: i64 = 0;
    for r in ranges {
        total += range_count_inclusive(r);
    }
    total
}

fn range_count_inclusive(r: &Range<i64>) -> i64 {
    r.end - r.start
}

fn ranges_check_and_add(r: Range<i64>, set: &mut Vec<Range<i64>>) {
    let mut to_remove = Vec::<usize>::new();
    let mut combined_range = r.clone();
    for i in 0..set.len() {
        if ranges_overlap(&combined_range, &set[i]) {
            to_remove.push(i);
            combined_range = ranges_combine(&combined_range, &set[i]);
        }
    }
    for i in to_remove.iter().rev() {
        set.remove(*i);
    }
    set.push(combined_range);
}

fn ranges_overlap(r1: &Range<i64>, r2: &Range<i64>) -> bool {
    r1.start <= r2.end && r1.end >= r2.start
}

fn ranges_combine(r1: &Range<i64>, r2: &Range<i64>) -> Range<i64> {
    let start: i64;
    let end: i64;
    if r1.start < r2.start {
        start = r1.start;
    }
    else {
        start = r2.start;
    }
    if r1.end > r2.end {
        end = r1.end;
    }
    else {
        end = r2.end;
    }
    Range { start: start, end: end }
}

fn parse_ranges(lines: &[&str]) -> Vec<Range<i64>> {
    let mut output = Vec::<Range<i64>>::new();
    for line in lines {
        output.push(get_range(line));
    }
    output
}

fn parse_items(lines: &[&str]) -> Vec<i64> {
    let mut output = Vec::<i64>::new();
    for line in lines {
        output.push(line.parse::<i64>().unwrap());
    }
    output
}

fn get_range(textform: &str) -> Range<i64> {
    let parts = Vec::from_iter(textform.split('-').map(|x| x.parse::<i64>().unwrap()));
    Range {  start: parts[0], end: parts[1] + 1 }
}

fn in_ranges_inclusive(val: i64, ranges: &Vec<Range<i64>>) -> bool {
    for r in ranges {
        if in_range_inclusive(val, r) {
            return true;
        }
    }
    false
}

fn in_range_inclusive(val: i64, range: &Range<i64>) -> bool {
    val >= range.start && val <= range.end
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_for_example_data() {
        let example_input = vec![
            Range { start: 3, end: 6},
            Range { start: 10, end: 15},
            Range { start: 16, end: 21},
            Range { start: 12, end: 19},
        ];

        let mut range_set = Vec::<Range<i64>>::new();
        for r in example_input {
            ranges_check_and_add(r,&mut range_set);
        }
        let test_result = ranges_sum(&range_set);

        assert_eq!(test_result, 14);
    }

    #[test]
    fn range_combine_1_precedes_2() {
        let range1 = Range::<i64> { start: 5, end: 10 };
        let range2 = Range::<i64> { start: 7, end: 15 };

        let test_output = ranges_combine(&range1, &range2);

        assert_eq!(5, test_output.start);
        assert_eq!(15, test_output.end);
    }

    #[test]
    fn range_combine_1_after_2() {
        let range2 = Range::<i64> { start: 5, end: 10 };
        let range1 = Range::<i64> { start: 7, end: 15 };

        let test_output = ranges_combine(&range1, &range2);

        assert_eq!(5, test_output.start);
        assert_eq!(15, test_output.end);
    }

    #[test]
    fn range_combine_1_superset_of_2() {
        let range1 = Range::<i64> { start: 5, end: 15 };
        let range2 = Range::<i64> { start: 7, end: 10 };

        let test_output = ranges_combine(&range1, &range2);

        assert_eq!(5, test_output.start);
        assert_eq!(15, test_output.end);
    }

    #[test]
    fn range_combine_1_subset_of_2() {
        let range2 = Range::<i64> { start: 5, end: 15 };
        let range1 = Range::<i64> { start: 7, end: 10 };

        let test_output = ranges_combine(&range1, &range2);

        assert_eq!(5, test_output.start);
        assert_eq!(15, test_output.end);
    }

    #[test]
    fn range_combine_1_precedes_2_ends_overlap() {
        let range1 = Range::<i64> { start: 5, end: 15 };
        let range2 = Range::<i64> { start: 7, end: 15 };

        let test_output = ranges_combine(&range1, &range2);

        assert_eq!(5, test_output.start);
        assert_eq!(15, test_output.end);
    }

        #[test]
    fn range_combine_2_precedes_1_ends_overlap() {
        let range2 = Range::<i64> { start: 5, end: 15 };
        let range1 = Range::<i64> { start: 7, end: 15 };

        let test_output = ranges_combine(&range1, &range2);

        assert_eq!(5, test_output.start);
        assert_eq!(15, test_output.end);
    }

    #[test]
    fn range_combine_1_after_2_starts_overlap() {
        let range2 = Range::<i64> { start: 5, end: 10 };
        let range1 = Range::<i64> { start: 5, end: 15 };

        let test_output = ranges_combine(&range1, &range2);

        assert_eq!(5, test_output.start);
        assert_eq!(15, test_output.end);
    }

    #[test]
    fn range_combine_2_after_1_starts_overlap() {
        let range1 = Range::<i64> { start: 5, end: 10 };
        let range2 = Range::<i64> { start: 5, end: 15 };

        let test_output = ranges_combine(&range1, &range2);

        assert_eq!(5, test_output.start);
        assert_eq!(15, test_output.end);
    }

        #[test]
    fn range_overlap_1_precedes_2() {
        let range1 = Range::<i64> { start: 5, end: 10 };
        let range2 = Range::<i64> { start: 7, end: 15 };

        let test_output = ranges_overlap(&range1, &range2);

        assert_eq!(test_output, true);
    }

    #[test]
    fn range_overlap_1_after_2() {
        let range2 = Range::<i64> { start: 5, end: 10 };
        let range1 = Range::<i64> { start: 7, end: 15 };

        let test_output = ranges_overlap(&range1, &range2);

        assert_eq!(test_output, true);
    }

    #[test]
    fn range_overlap_1_superset_of_2() {
        let range1 = Range::<i64> { start: 5, end: 15 };
        let range2 = Range::<i64> { start: 7, end: 10 };

        let test_output = ranges_overlap(&range1, &range2);

        assert_eq!(test_output, true);
    }

    #[test]
    fn range_overlap_1_subset_of_2() {
        let range2 = Range::<i64> { start: 5, end: 15 };
        let range1 = Range::<i64> { start: 7, end: 10 };

        let test_output = ranges_overlap(&range1, &range2);

        assert_eq!(test_output, true);
    }

    #[test]
    fn range_overlap_1_precedes_2_ends_overlap() {
        let range1 = Range::<i64> { start: 5, end: 15 };
        let range2 = Range::<i64> { start: 7, end: 15 };

        let test_output = ranges_overlap(&range1, &range2);

        assert_eq!(test_output, true);
    }

    #[test]
    fn range_overlap_2_precedes_1_ends_overlap() {
        let range2 = Range::<i64> { start: 5, end: 15 };
        let range1 = Range::<i64> { start: 7, end: 15 };

        let test_output = ranges_overlap(&range1, &range2);

        assert_eq!(test_output, true);
    }

    #[test]
    fn range_overlap_1_after_2_starts_overlap() {
        let range2 = Range::<i64> { start: 5, end: 10 };
        let range1 = Range::<i64> { start: 5, end: 15 };

        let test_output = ranges_overlap(&range1, &range2);

        assert_eq!(test_output, true);
    }

    #[test]
    fn range_overlap_2_after_1_starts_overlap() {
        let range1 = Range::<i64> { start: 5, end: 10 };
        let range2 = Range::<i64> { start: 5, end: 15 };

        let test_output = ranges_overlap(&range1, &range2);

        assert_eq!(test_output, true);
    }

    #[test]
    fn range_overlap_1_before_2_no_overlap() {
        let range1 = Range::<i64> { start: 5, end: 10 };
        let range2 = Range::<i64> { start: 233, end: 297 };

        let test_output = ranges_overlap(&range1, &range2);

        assert_eq!(test_output, false);
    }

    #[test]
    fn range_overlap_2_before_1_no_overlap() {
        let range1 = Range::<i64> { start: 5, end: 11 };
        let range2 = Range::<i64> { start: 233, end: 297 };

        let test_output = ranges_overlap(&range1, &range2);

        assert_eq!(test_output, false);
    }

    #[test]
    fn range_overlap_1_before_2_adjacent() {
        let range1 = Range::<i64> { start: 5, end: 11 };
        let range2 = Range::<i64> { start: 11, end: 297 };

        let test_output = ranges_overlap(&range1, &range2);

        assert_eq!(test_output, true);
    }

        #[test]
    fn range_overlap_1_before_2_gap() {
        let range1 = Range::<i64> { start: 5, end: 11 };
        let range2 = Range::<i64> { start: 12, end: 297 };

        let test_output = ranges_overlap(&range1, &range2);

        assert_eq!(test_output, false);
    }

        #[test]
    fn range_overlap_2_before_1_adjacent() {
        let range2 = Range::<i64> { start: 5, end: 11 };
        let range1 = Range::<i64> { start: 11, end: 297 };

        let test_output = ranges_overlap(&range1, &range2);

        assert_eq!(test_output, true);
    }

        #[test]
    fn range_overlap_2_before_1_gap() {
        let range2 = Range::<i64> { start: 5, end: 10 };
        let range1 = Range::<i64> { start: 12, end: 297 };

        let test_output = ranges_overlap(&range1, &range2);

        assert_eq!(test_output, false);
    }
}