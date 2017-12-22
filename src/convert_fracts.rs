// https://www.codewars.com/kata/54d7660d2daf68c619000d95/train/rust

pub fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut d = 1;

    loop {
        let mut result = vec![];

        let flag = l.iter().all(|&(i, j)| {
            let t = i * d;
            if t % j != 0 {
                return false;
            }

            result.push((t / j, d));
            true
        });
        if flag {
            return result;
        } else {
            d += 1;
        }
    }
}

fn testing(l: Vec<(i64, i64)>, exp: Vec<(i64, i64)>) -> () {
    assert_eq!(convert_fracts(l), exp)
}

#[test]
fn basics_convert_fracts() {
    testing(vec![(69, 130), (87, 1310), (3, 4)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
    testing(vec![(690, 1300), (87, 1310), (30, 40)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
}