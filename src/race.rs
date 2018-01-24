// https://www.codewars.com/kata/55e2adece53b4cdcb900006c/train/rust

fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v1 >= v2 {
        return None;
    }

    let s = g * 3600 / (v2 - v1);
    let hour = s / 3600;
    let min = s / 60 % 60;
    let sec = s % 60;
    Some(vec![hour, min, sec])
}

#[test]
fn basic_tests() {
    assert_eq!(race(720, 850, 70), Some(vec![0, 32, 18]));
    assert_eq!(race(80, 100, 40), Some(vec![2, 0, 0]));
    assert_eq!(race(80, 91, 37), Some(vec![3, 21, 49]));
    assert_eq!(race(820, 850, 550), Some(vec![18, 20, 0]));
    assert_eq!(race(820, 81, 550), None);
}