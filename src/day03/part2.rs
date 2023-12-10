fn filter_by<F>(input: Vec<&str>, n: usize, f: F) -> i32
where
    F: Fn(i32, i32) -> char,
{
    if input.len() == 1 {
        return i32::from_str_radix(input.first().unwrap(), 2).unwrap();
    }

    let mut zero_count = 0;
    let mut one_count = 0;

    for l in input.iter() {
        match l.chars().nth(n) {
            Some('0') => zero_count = zero_count + 1,
            Some('1') => one_count = one_count + 1,
            Some(x) => panic!("{}", x),
            None => panic!(),
        }
    }

    let criteria: char;
    criteria = f(zero_count, one_count);

    let filtered: Vec<&str> = input
        .into_iter()
        .filter(|l| l.chars().nth(n).unwrap() == criteria)
        .collect();
    filter_by(filtered, n + 1, f)
}

pub fn solve(input: &str) -> String {
    let i_oxy = filter_by(input.lines().collect::<Vec<&str>>(), 0, oxygen_criteria);
    let i_scr = filter_by(input.lines().collect::<Vec<&str>>(), 0, scrubber_criteria);

    (i_oxy * i_scr).to_string()
}

fn oxygen_criteria(zeros: i32, ones: i32) -> char {
    if zeros > ones {
        '0'
    } else {
        '1'
    }
}
fn scrubber_criteria(zeros: i32, ones: i32) -> char {
    if zeros > ones {
        '1'
    } else {
        '0'
    }
}
