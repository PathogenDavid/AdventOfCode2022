use std::collections::HashSet;

fn main() {
    let movements = include_str!("day9.txt").trim().replace("\r", "");

    println!("Part 1: {}", simulate(&movements, 1, 0));
    println!("Part 2: {}", simulate(&movements, 9, 0));
}

fn simulate(movements: &String, tail_count: usize, debug_level: i32) -> usize {
    let mut head: (i32, i32) = (0, 0);
    let mut tails: Vec<(i32, i32)> = vec![(0, 0); tail_count];
    let mut visited_spaces: HashSet<(i32, i32)> = HashSet::new();
    visited_spaces.insert((0, 0));

    if debug_level > 0 {
        println!("== Initial State ==");
        print_board(head, &tails);
    }

    for (direction, count) in movements.lines().map(|l| l.split_once(' ').unwrap()) {
        if debug_level > 0 {
            println!();
            println!("== {direction} {count} ==");
        }

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

            // Move the tails as needed
            let mut last_tail = head;
            for tail in tails.iter_mut() {
                if (last_tail.0 - tail.0).abs() > 1 || (last_tail.1 - tail.1).abs() > 1 {
                    if last_tail.0 > tail.0 {
                        tail.0 += 1;
                    } else if last_tail.0 < tail.0 {
                        tail.0 -= 1;
                    }

                    if last_tail.1 > tail.1 {
                        tail.1 += 1;
                    } else if last_tail.1 < tail.1 {
                        tail.1 -= 1;
                    }
                }

                last_tail = *tail;
            }

            // Record places the tail has visited
            visited_spaces.insert(*tails.last().unwrap());

            // Print the board
            if debug_level == 2 {
                print_board(head, &tails);
            }
        }

        if debug_level == 1 {
            print_board(head, &tails);
        }
    }

    if debug_level > 0 {
        println!();
    }

    visited_spaces.len()
}

#[allow(dead_code)]
fn print_board(head: (i32, i32), tails: &Vec<(i32, i32)>) {
    println!();
    //let (board_span_width, board_span_height) = (0..6, 0..5); // Sample
    let (board_span_width, board_span_height) = (-11..15, -5..16); // Sample 2

    for y in board_span_height.into_iter().rev() {
        'row_loop: for x in board_span_width.clone() {
            let pos = (x, y);
            if pos == head {
                print!("H");
                continue;
            }

            for (i, tail) in tails.iter().enumerate() {
                if pos == *tail {
                    if i == 0 && tails.len() == 1 {
                        print!("T");
                    } else {
                        print!("{}", i + 1)
                    }
                    continue 'row_loop;
                }
            }

            if pos == (0, 0) {
                print!("s");
            } else {
                print!(".");
            }
        }

        println!();
    }
}
