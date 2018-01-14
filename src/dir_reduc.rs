#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
}


fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut result: Vec<Direction> = vec![];
    for a in arr {
        if let Some(_last) = result.last().cloned() {
            if is_opposite(a, &_last) {
                result.pop().unwrap();
                continue;
            }
        }
        result.push(*a);
    }
    result
}

fn is_opposite(dir: &Direction, target: &Direction) -> bool {
    let op = &match *dir {
        Direction::NORTH => Direction::SOUTH,
        Direction::SOUTH => Direction::NORTH,
        Direction::EAST => Direction::WEST,
        Direction::WEST => Direction::EAST,
    };
    op == target
}

#[test]
fn returns_expected() {
    use self::Direction::*;
    let a = [NORTH, SOUTH, SOUTH, EAST, WEST, NORTH, WEST];
    assert_eq!(dir_reduc(&a), [WEST]);
    let a = [NORTH, WEST, SOUTH, EAST];
    assert_eq!(dir_reduc(&a), [NORTH, WEST, SOUTH, EAST]);
}