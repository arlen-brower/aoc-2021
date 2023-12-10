fn filter_by<F>(input: Vec<&str>, n: usize, f: F) -> &str
where
    F: Fn(i32, i32) -> char,
{
    if input.len() == 1 {
        return input.first().unwrap();
    }

    let criteria: char;
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

    criteria = f(zero_count, one_count);

    let filtered: Vec<&str> = input
        .into_iter()
        .filter(|l| l.chars().nth(n).unwrap() == criteria)
        .collect();
    filter_by(filtered, n + 1, f)
}

pub fn solve(input: &str) -> String {
    let oxy = filter_by(input.lines().collect::<Vec<&str>>(), 0, oxygen_criteria);
    let scrubber = filter_by(input.lines().collect::<Vec<&str>>(), 0, scrubber_criteria);
    let i_oxy = i32::from_str_radix(oxy, 2).unwrap();
    let i_scr = i32::from_str_radix(scrubber, 2).unwrap();

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
