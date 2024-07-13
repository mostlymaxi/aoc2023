use std::io::{BufRead, BufReader};

fn is_num(input: &str) -> bool {
    match input {
        "one" => true,
        _ => false,
    };
    input
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or_default()
}

fn to_num(input: &str) -> Option<usize> {
    match input {
        x if x.starts_with("one") => return Some(1),
        x if x.starts_with("two") => return Some(2),
        x if x.starts_with("three") => return Some(3),
        x if x.starts_with("four") => return Some(4),
        x if x.starts_with("five") => return Some(5),
        x if x.starts_with("six") => return Some(6),
        x if x.starts_with("seven") => return Some(7),
        x if x.starts_with("eight") => return Some(8),
        x if x.starts_with("nine") => return Some(9),
        _ => {}
    };

    if !input.is_empty() {
        return str::parse(&input[0..1]).ok();
    };

    None
}

fn main() {
    let f = std::fs::File::open("input").unwrap();
    let rdr = BufReader::new(f);
    let mut running_sum = 0;

    for line in rdr.lines() {
        let Ok(line) = line else {
            panic!();
            continue;
        };

        let mut idx_left = None;
        let mut idx_right = None;

        for i in 0..=line.len() {
            if idx_left.is_none() {
                idx_left = to_num(&line[i..]);
            }

            if idx_right.is_none() {
                idx_right = to_num(&line[line.len() - i..]);
            }

            if idx_left.is_some() && idx_right.is_some() {
                // eprintln!("{line}");
                // eprintln!("{:#?} {:#?}", idx_left, idx_right);
                break;
            }
        }

        if idx_left.is_none() || idx_right.is_none() {
            eprintln!("{line}")
        }

        running_sum += idx_left.unwrap_or(0) * 10 + idx_right.unwrap_or(0);
    }

    eprintln!("{running_sum}");
}

#[test]
fn test_match() {
    let input = "oneeeee";

    match input {
        x if x.starts_with("one") => eprintln!("ping"),
        _ => eprintln!("pong"),
    }
}
