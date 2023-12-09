pub fn solve(input: &str) -> String {
    let numbers: Vec<u32> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    let num_inc: u32 = numbers
        .windows(2)
        .map(|win| if win[1] > win[0] { 1 } else { 0 })
        .sum();

    num_inc.to_string()
}
