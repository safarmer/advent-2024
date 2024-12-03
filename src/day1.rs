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

pub fn day1b() {
    println!("Day 1a");
}

#[cfg(test)]
mod tests {
    use super::*;

    const IN_1A: &str = include_str!("data/1a.txt");

    fn day1_out(step: &str, i: i32) {
        println!("Day 1 - {:<6}: {}", step, i);
    }

    #[test]
    fn simple_input_1a() {
        let input = r#"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        "#;
        let lines: Vec<String> = input.lines().map(|i| i.trim().to_string()).collect();
        day1_out("sample", day1a(&lines));
    }

    #[test]
    fn day1a_1a() {
        let lines: Vec<String> = IN_1A.lines().map(|i| i.trim().to_string()).collect();
        day1_out("1a", day1a(&lines));
    }
}
