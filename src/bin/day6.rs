#![allow(unused)]

fn main() {
    let data_streams = include_str!("day6.txt").trim().replace("\r", "");

    for (i, data_stream) in data_streams.lines().enumerate() {
        println!(
            "{}{}",
            if i == 0 { "Part 1: " } else { "        " },
            match part1(data_stream) {
                Ok(x) => x.to_string(),
                Err(e) => e.to_string(),
            }
        );
    }
}

fn part1(data_stream: &str) -> Result<usize, &str> {
    let data_stream: Vec<_> = data_stream.chars().collect();
    for i in 3..data_stream.len() {
        let [a, b, c, d] = &data_stream[i - 3..=i] else { unreachable!(); };
        if a != b && a != c && a != d && b != c && b != d && c != d {
            return Ok(i + 1);
        }
    }

    Err("data stream is invalid")
}
