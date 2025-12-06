use std::fs;

pub fn second_day(){
    let mut sum_of_ids = 0;
    let ranges = parse_input();
    for range in ranges{
        let first_and_last = range.split("-").collect::<Vec<&str>>();
        let first = first_and_last.get(0).unwrap().parse::<i64>().unwrap();
        let last = first_and_last.get(1).unwrap().parse::<i64>().unwrap();
        sum_of_ids += find_incorrect_in_range(first, last);
    }
    println!("Sum of incorrect ids: {}", sum_of_ids);
}

fn parse_input() -> Vec<String>{
    let input = fs::read_to_string("inputs/second_input.txt").unwrap();
    let mut ranges: Vec<String> = Vec::new();
    for range in input.split(","){
        ranges.push(range.to_string());
    }
    ranges
}

fn find_incorrect_in_range(first: i64, last: i64) -> i64{
    let mut incorrect_ids = 0;
    for n in first..last+1{
        let n_str = n.to_string();
        let len_of_n = n_str.len();
        if len_of_n % 2 != 0{
            continue;
        }
        if n_str[..len_of_n/2] != n_str[len_of_n/2..]{
            continue;
        }
        incorrect_ids += n;
    }
    incorrect_ids
}