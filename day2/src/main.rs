use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

fn main() {
    let f = std::fs::File::open("input").unwrap();
    let rdr = BufReader::new(f);
    let mut running_sum = 0;
    let red = 12;
    let green = 13;
    let blue = 14;

    for line in rdr.lines() {
        let Ok(line) = line else {
            continue;
        };
        let mut hs = HashMap::new();

        let mut line_itr = line.split(' ');
        let id = line_itr
            .nth(1)
            .unwrap()
            .trim_end_matches(":")
            .parse::<usize>()
            .unwrap();

        while let Some(num) = line_itr.next() {
            let num = str::parse::<usize>(num).unwrap();

            let color = line_itr.next().unwrap();
            let color = color.trim_end_matches(";");
            let color = color.trim_end_matches(",");
            let color = color.to_string();

            hs.entry(color)
                .and_modify(|running_max| *running_max = std::cmp::max(*running_max, num))
                .or_insert(num);
        }

        running_sum += hs.get("red").unwrap_or(&1)
            * hs.get("green").unwrap_or(&1)
            * hs.get("blue").unwrap_or(&1);
    }

    eprintln!("{running_sum}");
}
