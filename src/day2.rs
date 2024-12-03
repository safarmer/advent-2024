use std::collections::HashMap;
use std::string;

fn check(lines: &Vec<String>, is_safe: fn(&[i32]) -> bool) -> i32 {
    let to_int = |s: &str| s.trim().parse::<i32>().unwrap();

    lines
        .iter()
        .map(|s| s.trim_ascii())
        .filter(|s| !s.is_empty())
        .map(|l| l.split_whitespace().map(to_int).collect::<Vec<i32>>())
        .filter(|x| is_safe(x.as_slice()))
        .count() as i32
}

pub fn day2a(lines: &Vec<String>) -> i32 {
    fn is_safe(report: &[i32]) -> bool {
        let fact = if report[0] < report[1] { 1 } else { -1 };
        for i in 1..report.len() {
            let step = (report[i] - report[i - 1]) * fact;
            if step <= 0 || step > 3 {
                return false;
            }
        }
        true
    }

    check(lines, is_safe)
}

pub fn day2b(lines: &Vec<String>) -> i32 {
    fn is_safe(report: &[i32]) -> bool {
        // TODO(shane): check safety for part 2b.
        false
    }

    check(lines, is_safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    const IN_SAMPLE: &str = r#"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
        "#;

    const IN_FILE: &str = include_str!("data/day2.txt");

    fn day2_out(step: &str, i: i32) {
        println!("{:<10}: {}", step, i);
    }

    #[test]
    fn day2a_sample() {
        let lines: Vec<String> = IN_SAMPLE.lines().map(|i| i.trim().to_string()).collect();
        let out = day2a(&lines);
        day2_out("2a sample", out);
        assert_eq!(out, 2);
    }

    #[test]
    fn day2a_input() {
        let lines: Vec<String> = IN_FILE.lines().map(|i| i.trim().to_string()).collect();
        day2_out("2a input", day2a(&lines));
    }

    #[test]
    fn day2b_sample() {
        let lines: Vec<String> = IN_SAMPLE.lines().map(|i| i.trim().to_string()).collect();
        let out = day2b(&lines);
        day2_out("2b sample", out);
        assert_eq!(out, 4);
    }

    #[test]
    fn day2b_input() {
        let lines: Vec<String> = IN_FILE.lines().map(|i| i.trim().to_string()).collect();
        day2_out("2b input", day2b(&lines));
    }
}
