fn prime_factors(n: i64) -> String {
    let mut sum = n;
    let mut cur = 2;
    let mut result = "".to_string();

    while cur <= sum {
        let mut count = 0;
        while sum % cur == 0 {
            sum /= cur;
            count += 1;
        }
        if count > 0 {
            let s = if count == 1 {
                format!("({})", cur)
            } else {
                format!("({}**{})", cur, count)
            };
            result.push_str(s.as_str());
        }
        cur += 1;
    }
    result
}

fn testing(n: i64, exp: &str) -> () {
    assert_eq!(&prime_factors(n), exp)
}

#[test]
fn basics_prime_factors() {
    testing(7919, "(7919)");
    testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
    testing(17 * 17 * 93 * 677, "(3)(17**2)(31)(677)");
}