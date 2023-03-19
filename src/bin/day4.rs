fn main() {
    let assignments = include_str!("day4.txt").trim().replace("\r", "");
    println!("Part 1: {}", part1(&assignments));
    println!("Part 2: {}", part2(&assignments));
}

fn part1(assignments: &str) -> usize {
    assignments.lines()
        // Parse lines to range pairs
        .map(|pair| {
            pair.split(",").map(|range| {
                range.split("-").map(|i| i.parse::<i32>().unwrap()).pair()
            }).pair()
        })
        // Filter to pairs where one range fits within another
        //                            (a fits in b) or (b fits in a)
        .filter(|(a, b)| (a.0 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.1 <= a.1))
        .count()
}

fn part2(assignments: &str) -> usize {
    assignments.lines()
        // Parse lines to range pairs
        .map(|pair| {
            pair.split(",").map(|range| {
                range.split("-").map(|i| i.parse::<i32>().unwrap()).pair()
            }).pair()
        })
        // Filter to pairs where ranges overlap
        //                  (a's start is within b) or (b's start is wthin a)
        .filter(|(a, b)| (a.0 >= b.0 && a.0 <= b.1) || (b.0 >= a.0 && b.0 <= a.1))
        .count()
}

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
