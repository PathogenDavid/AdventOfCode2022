use std::collections::HashSet;

fn main() {
    let data_streams = include_str!("day6.txt").trim().replace("\r", "");

    println!("Part 1 / Part 2");
    for data_stream in data_streams.lines() {
        println!(
            "{} / {}",
            match find_marker(data_stream, 4) {
                Ok(x) => x.to_string(),
                Err(e) => e.to_string(),
            },
            match find_marker(data_stream, 14) {
                Ok(x) => x.to_string(),
                Err(e) => e.to_string(),
            }
        );
    }
}

fn find_marker(data_stream: &str, marker_len: usize) -> Result<usize, &str> {
    let data_stream: Vec<_> = data_stream.chars().collect();
    let mut check = HashSet::with_capacity(marker_len);
    let offset = marker_len - 1;

    for i in offset..data_stream.len() {
        check.clear();

        // If all characters are unique, we found the message
        if data_stream[i - offset..=i].iter().all(|c| check.insert(c)) {
            return Ok(i + 1);
        }
    }

    Err("data stream is invalid")
}
