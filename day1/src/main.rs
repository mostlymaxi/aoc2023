use std::io::{BufRead, BufReader};

fn is_num(input: &str) -> bool {
    input
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or_default()
}

fn to_num(input: &str) -> usize {
    str::parse(input).unwrap_or_default()
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

        for i in 0..line.len() {
            if is_num(&line[i..]) && idx_left.is_none() {
                idx_left = Some(to_num(&line[i..i + 1]) * 10);
            }

            if is_num(&line[line.len() - i..]) && idx_right.is_none() {
                idx_right = Some(to_num(&line[line.len() - i..line.len() - i + 1]));
            }

            if idx_left.is_some() && idx_right.is_some() {
                eprintln!("{line}");
                eprintln!("{:#?} {:#?}", idx_left, idx_right);
                break;
            }
        }

        running_sum += idx_left.unwrap_or(0) + idx_right.unwrap_or(0);
    }

    eprintln!("{running_sum}");
}
