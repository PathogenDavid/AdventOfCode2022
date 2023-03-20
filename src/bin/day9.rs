use std::collections::HashSet;

//#[allow(unused)]

fn main() {
    let movements = include_str!("day9.txt").trim().replace("\r", "");

    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut visited_spaces: HashSet<(i32, i32)> = HashSet::new();
    visited_spaces.insert(tail);

    //println!("== Initial State ==");
    //print_board(head, tail);

    for (direction, count) in movements.lines().map(|l| l.split_once(' ').unwrap()) {
        //println!();
        //println!("== {direction} {count} ==");

        let count: i32 = count.parse().unwrap();
        let direction: (i32, i32) = match direction {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!("direction '{direction}' is invalid"),
        };

        for _ in 0..count {
            // Move the head
            head.0 += direction.0;
            head.1 += direction.1;

            // Move the tail if necessary
            if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                // Axis-aligned movement
                if head.0 == tail.0 || head.1 == tail.1 {
                    tail.0 += direction.0;
                    tail.1 += direction.1;
                } else { // Diagonal movement
                    if head.0 > tail.0 {
                        tail.0 += 1;
                    } else {
                        tail.0 -= 1;
                    }

                    if head.1 > tail.1 {
                        tail.1 += 1;
                    } else {
                        tail.1 -= 1;
                    }
                }
            }

            // Record places the tail has visited
            visited_spaces.insert(tail);

            // Print the board
            //print_board(head, tail);
        }
    }

    println!();
    println!("Part 1: {}", visited_spaces.len());
}

#[allow(dead_code)]
fn print_board(head: (i32, i32), tail: (i32, i32)) {
    println!();
    const BOARD_SIZE: (i32, i32) = (6, 5);

    for y in (0..BOARD_SIZE.1).into_iter().rev() {
        for x in 0..BOARD_SIZE.0 {
            let pos = (x, y);
            if pos == head {
                print!("H");
            } else if pos == tail {
                print!("T");
            } else if pos == (0, 0) {
                print!("s");
            } else {
                print!(".");
            }
        }

        println!();
    }
}
