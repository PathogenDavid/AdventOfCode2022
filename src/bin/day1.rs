fn main() {
    let calories = include_str!("day1.txt").trim().replace("\r", "");

    // Parse input into groups by elf and then convert+sum each elf
    let calories_per_elf = calories.split("\n\n").map(|calorie_list| {
        calorie_list
            .lines()
            .map(|s| s.parse::<u32>().expect("input must consist of numbers"))
            .sum::<u32>()
    });

    // Find the elf with the most calories
    let solution = calories_per_elf.clone().max().unwrap_or(0);
    println!("Part 1: {solution}");

    //===============================================================================================================================================
    // Part 2
    //===============================================================================================================================================

    // Convert the calories per elf to a sorted list and grab the top 3 results
    let mut calories_per_elf: Vec<_> = calories_per_elf.collect();
    calories_per_elf.sort();
    let solution: u32 = calories_per_elf.iter().rev().take(3).sum();
    println!("Part 2: {solution}");
}
