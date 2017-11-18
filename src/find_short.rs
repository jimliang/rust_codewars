fn find_short(s: &str) -> u32 {
    s.split_whitespace().map(str::len).min().unwrap() as u32
}

#[test]
fn returns_expected() {
    assert_eq!(find_short("bitcoin take over the world maybe who knows perhaps"), 3);
    assert_eq!(find_short("turns out random test cases are easier than writing out basic ones"), 3);
    assert_eq!(find_short("lets talk about javascript the best language"), 3);
    assert_eq!(find_short("i want to travel the world writing code one day"), 1);
    assert_eq!(find_short("Lets all go on holiday somewhere very cold"), 2);
}