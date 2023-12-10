pub fn solve(input: &str) -> String {
    let width = 12; // Real input
                    //let width = 5; // Test input
    let mut gamma = String::new();
    let mut epsilon = String::new();

    for i in 0..width {
        let mut zero_count = 0;
        let mut one_count = 0;
        for l in input.lines() {
            match l.chars().nth(i) {
                Some('0') => zero_count = zero_count + 1,
                Some('1') => one_count = one_count + 1,
                Some(_) => panic!(),
                None => panic!(),
            }
        }

        if zero_count > one_count {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }
    let i_gam = u32::from_str_radix(gamma.as_str(), 2).unwrap();
    let i_eps = u32::from_str_radix(epsilon.as_str(), 2).unwrap();
    let answer = i_gam * i_eps;
    answer.to_string()
}
