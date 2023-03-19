use std::ops::RangeInclusive;

fn main() {
    let assignments = include_str!("day4.txt").trim().replace("\r", "");
    println!("Part 1: {}", part1(&assignments));
    println!("Part 2: {}", part2(&assignments));
    println!("Part 2: {} (Alt solution)", part2_alt(&assignments));
}

fn part1(assignments: &str) -> usize {
    assignments
        .lines()
        // Parse lines to range pairs
        .map(|pair| {
            pair.split(",")
                .map(|range| range.split("-").map(|i| i.parse::<i32>().unwrap()).pair())
                .pair()
        })
        // Filter to pairs where one range fits within another
        //                            (a fits in b) or (b fits in a)
        .filter(|(a, b)| (a.0 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.1 <= a.1))
        .count()
}

fn part2(assignments: &str) -> usize {
    assignments
        .lines()
        // Parse lines to range pairs
        .map(|pair| {
            pair.split(",")
                .map(|range| range.split("-").map(|i| i.parse::<i32>().unwrap()).pair())
                .pair()
        })
        // Filter to pairs where ranges overlap
        //                  (a's start is within b) or (b's start is wthin a)
        .filter(|(a, b)| (a.0 >= b.0 && a.0 <= b.1) || (b.0 >= a.0 && b.0 <= a.1))
        .count()
}

//-------------------------------------------------------------------------------------------------
// Pair extension for Iterator used for non-alt soltuions (because I didn't know about `split_once`
//-------------------------------------------------------------------------------------------------

trait Pair {
    type Item;

    /// Processes an iterator into just a tuple pair.
    /// Panics if iterator does not yield exactly two items.
    fn pair(self) -> (Self::Item, Self::Item);
}

impl<T: Iterator> Pair for T {
    type Item = T::Item;

    fn pair(mut self) -> (Self::Item, Self::Item) {
        let a = self.next().expect("iterator yielded no items");
        let b = self.next().expect("iterator yielded only one item");
        assert!(self.next().is_none(), "iterator yielded more than two items");
        (a, b)
    }
}

//-------------------------------------------------------------------------------------------------
// Part 2 alternate solution
//-------------------------------------------------------------------------------------------------

// I did not know about split_once or Option<T>::map, thanks again Chevy!
fn part2_alt(assignments: &str) -> usize {
    fn parse_range(range: &str) -> RangeInclusive<i32> {
        range
            .split_once('-')
            .map(|(a, b)| a.parse().unwrap()..=b.parse().unwrap())
            .unwrap()
    }

    assignments
        .lines()
        // Parse lines to range pairs
        .map(|pair| pair.split_once(',').unwrap())
        .map(|(a, b)| (parse_range(a), parse_range(b)))
        // Filter to pairs where ranges overlap
        //             (a's start is within b) or (b's start is wthin a)
        .filter(|(a, b)| a.contains(b.start()) || b.contains(a.start()))
        .count()
}
