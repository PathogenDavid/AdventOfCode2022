// I should've made a little 2D grid data structure but it's too late now :(
fn main() {
    //=============================================================================================
    // Parse map data
    //=============================================================================================
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

    //=============================================================================================
    // Calculate tree visibility
    //=============================================================================================
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

    //=============================================================================================
    // Part 1: Count the number of visible trees
    //=============================================================================================
    let total_visible: usize = trees
        .iter()
        .map(|r| r.iter().filter(|t| t.is_visible).count())
        .sum();
    println!("Part 1: {total_visible}");

    //=============================================================================================
    // Part 2: Find the tree with the highest scenic score
    //=============================================================================================
    let mut highest_score = 0;
    for row_num in 0..num_rows {
        for col_num in 0..num_cols {
            let scenic_score
                = calculate_partial_scenic_score(&trees, (row_num, col_num), (1, 0), (num_rows, num_cols))
                * calculate_partial_scenic_score(&trees, (row_num, col_num), (-1, 0), (num_rows, num_cols))
                * calculate_partial_scenic_score(&trees, (row_num, col_num), (0, 1), (num_rows, num_cols))
                * calculate_partial_scenic_score(&trees, (row_num, col_num), (0, -1), (num_rows, num_cols))
            ;
            highest_score = u32::max(highest_score, scenic_score);
        }
    }

    println!("Part 2: {highest_score}");
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

fn calculate_partial_scenic_score(
    trees: &Vec<Vec<Tree>>,
    start: (usize, usize),
    direction: (isize, isize),
    counts: (usize, usize),
) -> u32 {
    let mut row = start.0;
    let mut col = start.1;
    let check_height = trees[row][col].height;
    let mut score = 0;

    loop {
        row = row.wrapping_add_signed(direction.0);
        col = col.wrapping_add_signed(direction.1);

        // We are done if row or column hit the limit (or wrapped from 0 to above the limit)
        if row >= counts.0 || col >= counts.1 {
            break;
        }

        // Another tree another point!
        score += 1;

        // If this tree is the tallest tree we can see in this direction, we are done
        if trees[row][col].height >= check_height {
            break;
        }
    }

    score
}

struct Tree {
    height: i8,
    is_visible: bool,
}
