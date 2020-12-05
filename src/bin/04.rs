use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::iter::Iterator;

fn check_range(s: &str, l: i32, r: i32) -> bool {
    let x = s.parse::<i32>().unwrap();
    x >= l && x <= r
}
fn main() {
    let content = fs::read_to_string("input/4.txt").expect("Input file not found.");

    let entry_names = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let entry_rules = [
        |s: &str| check_range(s, 1920, 2002),
        |s: &str| check_range(s, 2010, 2020),
        |s: &str| check_range(s, 2020, 2030),
        |s: &str| {
            if s.ends_with("cm") {
                check_range(&s[..(s.len() - 2)], 150, 193)
            } else if s.ends_with("in") {
                check_range(&s[..s.len() - 2], 59, 76)
            } else {
                false
            }
        },
        |s: &str| {
            return s.len() == 7
                && s.starts_with("#")
                && s[1..]
                    .chars()
                    .all(|c| c.is_numeric() || c >= 'a' && c <= 'f');
        },
        |s: &str| {
            return vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .into_iter()
                .collect::<HashSet<&str>>()
                .contains(s);
        },
        |s: &str| return s.len() == 9 && s.chars().all(char::is_numeric),
    ];

    let (mut total0, mut total1) = (0, 0);

    for pass_str in content.split("\r\n\r\n") {
        let s = pass_str.replace("\r\n", " ");

        let mut passport = HashMap::new();

        for field in s.split(" ") {
            let parts: Vec<&str> = field.split(":").collect();
            if parts.len() == 2 {
                passport.insert(parts[0], parts[1]);
            }
        }

        let (mut good0, mut good1) = (true, true);
        for i in 0..entry_names.len() {
            match passport.get(entry_names[i]) {
                Some(&value) => {
                    if !entry_rules[i](value) {
                        good1 = false;
                    }
                }
                _ => {
                    good1 = false;
                    good0 = false
                }
            }
        }

        if good1 {
            println!("good: {:?}", s);
            total1 += 1;
        }

        if good0 {
            total0 += 1;
        }
    }
    println!("answer0: {:?}", total0);
    println!("answer1: {:?}", total1);
}
