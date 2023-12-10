fn parse_line((a_pos, a_dep): (i32, i32), line: &str) -> (i32, i32) {
    let (instr, num) = line.split_once(char::is_whitespace).unwrap();
    match instr.chars().nth(0).unwrap() {
        'f' => (a_pos + num.parse::<i32>().unwrap(), a_dep),
        'd' => (a_pos, a_dep + num.parse::<i32>().unwrap()),
        'u' => (a_pos, a_dep - num.parse::<i32>().unwrap()),
        _ => panic!(),
    }
}

pub fn solve(input: &str) -> String {
    let (pos, dep) = input.lines().fold((0, 0), |acc, l| parse_line(acc, l));

    (pos * dep).to_string()
}
