use std::collections::HashMap;

pub fn day1a(lines: &Vec<String>) -> i32 {
    let to_int = |s: &str| s.trim().parse::<i32>().unwrap();

    let mut ls: Vec<i32> = vec![];
    let mut rs: Vec<i32> = vec![];

    for line in lines {
        if let Some((l, r)) = line.split_once(' ').map(|(x, y)| (to_int(x), to_int(y))) {
            ls.push(l);
            rs.push(r);
        }
    }

    ls.sort();
    rs.sort();

    let mut sum = 0;
    for i in 0..ls.len() {
        sum += (ls[i] - rs[i]).abs();
    }
    sum
}

pub fn day1b(lines: &Vec<String>) -> i32 {
    let to_int = |s: &str| s.trim().parse::<i32>().unwrap();

    let mut ls: Vec<i32> = vec![];
    let mut rs = HashMap::new();

    for line in lines {
        if let Some((l, r)) = line.split_once(' ').map(|(x, y)| (to_int(x), to_int(y))) {
            ls.push(l);
            rs.entry(r).and_modify(|x| *x += 1).or_insert(1);
        }
    }


    ls.iter().map(|l| {l * rs.get(&l).map_or(0, |x| *x) }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const IN_SAMPLE: &str = r#"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        "#;

    const IN_FILE: &str = include_str!("data/day1.txt");

    fn day1_out(step: &str, i: i32) {
        println!("{:<10}: {}", step, i);
    }

    #[test]
    fn day1a_sample() {
        let lines: Vec<String> = IN_SAMPLE.lines().map(|i| i.trim().to_string()).collect();
        let out = day1a(&lines);
        day1_out("1a sample", out);
        assert_eq!(out, 11);
    }

    #[test]
    fn day1a_input() {
        let lines: Vec<String> = IN_FILE.lines().map(|i| i.trim().to_string()).collect();
        day1_out("1a input", day1a(&lines));
    }

    #[test]
    fn day1b_sample() {
        let lines: Vec<String> = IN_SAMPLE.lines().map(|i| i.trim().to_string()).collect();
        let out = day1b(&lines);
        day1_out("1b sample", out);
        assert_eq!(out, 31);
    }

    #[test]
    fn day1b_input() {
        let lines: Vec<String> = IN_FILE.lines().map(|i| i.trim().to_string()).collect();
        day1_out("1b input", day1b(&lines));
    }
}
