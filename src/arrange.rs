fn arrange(s: &str) -> String {
    let mut list: Vec<&str> = s.split(' ').collect();
    for i in 0..list.len() {
        let swap = i < list.len() - 1 && if i % 2 == 0 {
            list[i].len() > list[i + 1].len()
        } else {
            list[i].len() < list[i + 1].len()
        };
        if swap {
            list.swap(i, i + 1);
        }
    }
    let l: Vec<String> = list.iter().enumerate().map(|(idx, &s)| {
        if idx % 2 == 0 {
            s.to_lowercase()
        } else {
            s.to_uppercase()
        }
    }).collect();
    l.join(" ")
}


fn testing(s: &str, exp: &str) -> () {
    assert_eq!(arrange(s), exp.to_string())
}

#[test]
fn basics_arrange() {
    testing("who hit retaining The That a we taken", "who RETAINING hit THAT a THE we TAKEN"); // 3
    testing("on I came up were so grandmothers", "i CAME on WERE up GRANDMOTHERS so"); // 4
}