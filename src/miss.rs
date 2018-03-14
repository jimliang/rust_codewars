// http://www.codewars.com/kata/moves-in-squared-strings-iii

fn transform<F: FnMut(usize, usize, &Vec<Vec<char>>) -> char>(s: &str, mut f: F) -> Vec<String> {
    use std::iter::FromIterator;

    let lines: Vec<Vec<char>> = s
        .split('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            String::from_iter(line
                .iter()
                .enumerate()
                .map(|(j, c)| f(i, j, &lines))
            )
        })
        .collect()
}

fn diag_1_sym(s: &str) -> String {
    transform(s, |i, j, lines| lines[j][i]).join("\n")
}

fn rot_90_clock(s: &str) -> String {
    transform(s, |i, j, lines| lines[lines[j].len() - j - 1][i]).join("\n")
}

fn selfie_and_diag1(s: &str) -> String {
    s.split("\n")
        .zip(transform(s, |i, j, lines| lines[j][i]).iter())
        .map(|(a, b)| format!("{}|{}", a, b))
        .collect::<Vec<String>>()
        .join("\n")
}

// first parameter: dots have to be replaced by function of one variable
fn oper<F: FnOnce(&str) -> String>(f: F, s: &str) -> String {
    f(s)
}

fn testing1(s: &str, exp: &str) -> () {
    assert_eq!(oper(diag_1_sym, s), exp.to_string())
}

fn testing2(s: &str, exp: &str) -> () {
    assert_eq!(oper(rot_90_clock, s), exp.to_string())
}

fn testing3(s: &str, exp: &str) -> () {
    assert_eq!(oper(selfie_and_diag1, s), exp.to_string())
}

#[test]
fn basics_oper() {
    testing2("rgavce\nvGcEKl\ndChZVW\nxNWgXR\niJBYDO\nSdmEKb",
             "Sixdvr\ndJNCGg\nmBWhca\nEYgZEv\nKDXVKc\nbORWle");

    testing1("wuUyPC\neNHWxw\nehifmi\ntBTlFI\nvWNpdv\nIFkGjZ",
             "weetvI\nuNhBWF\nUHiTNk\nyWflpG\nPxmFdj\nCwiIvZ");

    testing3("NJVGhr\nMObsvw\ntPhCtl\nsoEnhi\nrtQRLK\nzjliWg",
             "NJVGhr|NMtsrz\nMObsvw|JOPotj\ntPhCtl|VbhEQl\nsoEnhi|GsCnRi\nrtQRLK|hvthLW\nzjliWg|rwliKg");
}