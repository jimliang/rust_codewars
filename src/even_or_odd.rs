
// http://www.codewars.com/kata/53da3dbb4a5168369a0000fe

fn even_or_odd(i: i32) -> &'static str {
    if i % 2 == 0 {
        "Even"
    } else {
        "Odd"
    }
}

#[test]
fn returns_expected() {
    assert_eq!(even_or_odd(0), "Even");
    assert_eq!(even_or_odd(2), "Even");
    assert_eq!(even_or_odd(1), "Odd");
    assert_eq!(even_or_odd(7), "Odd");
}