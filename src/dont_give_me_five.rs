fn dont_give_me_five(start: isize, end: isize) -> isize {
    (start..end + 1).filter(|&i| !i.to_string().contains('5')).count() as isize
}

#[test]
fn returns_expected() {
    assert_eq!(dont_give_me_five(1, 9), 8);
    assert_eq!(dont_give_me_five(4, 17), 12);
}