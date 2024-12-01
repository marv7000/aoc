fn main() {
    let list = include_str!("input.txt")
        .lines()
        .map(|x| {
            x.split("   ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut list1 = list.iter().map(|x| x[0]).collect::<Vec<_>>();
    list1.sort();

    let mut list2 = list.iter().map(|x| x[1]).collect::<Vec<_>>();
    list2.sort();

    println!(
        "Part 1: {}",
        list1
            .iter()
            .enumerate()
            .map(|(idx, y)| list2[idx].abs_diff(*y))
            .sum::<usize>()
    );

    println!(
        "Part 2: {}",
        list1
            .iter()
            .map(|y| *y * list2.iter().filter(|a| **a == *y).count())
            .sum::<usize>()
    );
}
