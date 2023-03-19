use std::collections::HashSet;

fn main() {
    let inventory = include_str!("day3.txt").trim().replace("\r", "");

    let mut priority_sum = 0;

    for rucksack in inventory.lines() {
        assert!(rucksack.len() % 2 == 0);
        let split = rucksack.len() / 2;
        let compartment1: HashSet<_> = rucksack[..split].chars().collect();
        let compartment2: HashSet<_> = rucksack[split..].chars().collect();

        for item in compartment2 {
            if compartment1.contains(&item) {
                priority_sum += get_item_priority(item);
            }
        }
    }

    println!("Part 1: {priority_sum}");
}

fn get_item_priority(item: char) -> u32 {
    match item {
        'a'..='z' => u32::from(item) - u32::from('a') + 1,
        'A'..='Z' => u32::from(item) - u32::from('A') + 27,
        _ => panic!("{item:?} is not a valid item identifier!"),
    }
}
