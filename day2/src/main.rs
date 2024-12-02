fn get_valid(input: impl Iterator<Item = Vec<usize>>) -> usize {
    input
        .filter(|x| {
            if !(x.is_sorted_by(|a, b| a <= b) || x.is_sorted_by(|a, b| a >= b)) {
                return false;
            }
            let mut x = x.clone();
            let mut iter = x.iter_mut();
            let mut last = *iter.next().unwrap();
            return iter.all(|z| {
                let is_valid = last.abs_diff(*z) >= 1 && last.abs_diff(*z) <= 3;
                last = *z;
                is_valid
            });
        })
        .count()
}

fn main() {
    let input = include_str!("input.txt").lines().map(|x| {
        x.split_ascii_whitespace()
            .map(|y| y.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    });

    println!("part 1: {}", get_valid(input.clone()));
}
