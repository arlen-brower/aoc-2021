pub fn solve(input: &str) -> String {
    let numbers: Vec<u32> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();

    let threes: Vec<u32> = numbers
        .windows(3)
        .map(|win| win[0] + win[1] + win[2])
        .collect();

    let answer: u32 = threes
        .windows(2)
        .map(|win| if win[1] > win[0] { 1 } else { 0 })
        .sum();

    answer.to_string()
}
