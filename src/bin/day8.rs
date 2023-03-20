#![allow(unused)]

use std::ops::Range;

fn main() {
    let map = include_str!("day8.txt").trim().replace("\r", "");

    let mut trees: Vec<_> = map
        .lines()
        .map(|line| {
            line.bytes()
                .map(|c| Tree {
                    height: (c as u8 - b'0') as i8,
                    is_visible: false,
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let num_rows = trees.len();
    let num_cols = trees.first().unwrap().len();
    assert!(trees.iter().all(|r| r.len() == num_cols));

    // Calclate east-west visibility
    for row in trees.iter_mut() {
        check_east_west(row.iter_mut());
        check_east_west(row.iter_mut().rev());
    }

    // Calculate north-south visibility
    for col in 0..num_cols {
        check_north_south(trees.iter_mut(), col);
        check_north_south(trees.iter_mut().rev(), col);
    }

    // Part 1: Count the number of visible trees
    let total_visible: usize = trees.iter().map(|r| r.iter().filter(|t| t.is_visible).count()).sum();
    println!("Part 1: {total_visible}");
}

fn check_east_west<'a, I>(iterator: I)
where
    I: Iterator<Item = &'a mut Tree>,
{
    let mut tallest_tree = -1;
    for tree in iterator {
        if tree.height > tallest_tree {
            tree.is_visible = true;
            tallest_tree = tree.height;
        }
    }
}

fn check_north_south<'a, I>(row_iterator: I, col: usize)
where
    I: Iterator<Item = &'a mut Vec<Tree>>,
{
    let mut tallest_tree = -1;
    for row in row_iterator {
        let tree = &mut row[col];
        if tree.height > tallest_tree {
            tree.is_visible = true;
            tallest_tree = tree.height;
        }
    }
}

struct Tree {
    height: i8,
    is_visible: bool,
}
