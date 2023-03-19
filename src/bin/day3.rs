use std::collections::HashSet;

fn main() {
    let inventory = include_str!("day3.txt").trim().replace("\r", "");
    println!("Part 1: {}", part1(&inventory));
    println!("Part 2: {}", part2(&inventory));
    println!("Part 2: {} (Alt solution)", part2_alt(&inventory));
}

fn part1(inventory: &str) -> u32 {
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

    priority_sum
}

fn part2(inventory: &str) -> u32 {
    let mut priority_sum = 0;

    let mut lines = inventory.lines();
    while let (Some(rucksack1), rucksack2, rucksack3) = (lines.next(), lines.next(), lines.next()) {
        // Take inventory of the first elf's rucksack
        let rucksack1: HashSet<_> = rucksack1.chars().collect();
        // Take inventory of the second elf's rucksack, only counting items that were found in the first elf's rucksack
        let rucksack2: HashSet<_> = rucksack2.unwrap().chars().filter(|item| rucksack1.contains(item)).collect();
        // And so on for the third elf, expecting to find a single item
        // (We enumerate the whole set because the elf might have more than one of the item)
        let rucksack3: HashSet<_> = rucksack3.unwrap().chars().filter(|item| rucksack2.contains(item)).collect();

        assert_eq!(rucksack3.len(), 1, "the three elves should only have a single item in common");
        priority_sum += get_item_priority(*rucksack3.iter().next().unwrap());
    }

    priority_sum
}

/// Tweaked solution to part 2 that uses `chunks_exact` after seeing it in @ChevyRay's solution
/// (I tried looking for it before but saw `Iterator::array_chunks` was nightly-only and didn't think to look at `slice`)
fn part2_alt(inventory: &str) -> u32 {
    let mut priority_sum = 0;

    let lines: Vec<_> = inventory.lines().collect();
    let groups = lines.chunks_exact(3);
    assert_eq!(groups.remainder().len(), 0, "input should not contain partial groups");

    for group in groups {
        // Take inventory of the first elf's rucksack
        let rucksack1: HashSet<_> = group[0].chars().collect();
        // Take inventory of the second elf's rucksack, only counting items that were found in the first elf's rucksack
        let rucksack2: HashSet<_> = group[1].chars().filter(|item| rucksack1.contains(item)).collect();
        // And so on for the third elf, expecting to find a single item
        // (We enumerate the whole set because the elf might have more than one of the item)
        let rucksack3: HashSet<_> = group[2].chars().filter(|item| rucksack2.contains(item)).collect();

        assert_eq!(rucksack3.len(), 1, "the three elves should only have a single item in common");
        priority_sum += get_item_priority(*rucksack3.iter().next().unwrap());
    }

    priority_sum
}

fn get_item_priority(item: char) -> u32 {
    match item {
        'a'..='z' => u32::from(item) - u32::from('a') + 1,
        'A'..='Z' => u32::from(item) - u32::from('A') + 27,
        _ => panic!("{item:?} is not a valid item identifier!"),
    }
}
