fn main() {
    let program = include_str!("day10.txt").trim().replace("\r", "");

    let mut cycle = 0;
    let mut x = 1;
    let mut signal_sum = 0;
    let sample_points = [20, 60, 100, 140, 180, 220];

    for instruction in program.lines() {
        let (cost, add_x) = if instruction == "noop" {
            (1, 0)
        } else {
            assert!(instruction.starts_with("addx "));
            (2, instruction.split_once(' ').unwrap().1.parse().unwrap())
        };

        // Simulate the instruction
        for _ in 0..cost {
            cycle += 1;

            if sample_points.contains(&cycle) {
                signal_sum += cycle * x;
            }
        }

        x += add_x;
    }

    println!("Part 1: {signal_sum}");
}
