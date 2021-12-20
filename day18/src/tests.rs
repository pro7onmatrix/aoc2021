use crate::snailfish::SnailfishNumber;

#[test]
fn test_construction_1() {
    let number = SnailfishNumber::new("[1,2]");
    assert_eq!(format!("{}", number), "[1,2]");
}

#[test]
fn test_construction_2() {
    let number = SnailfishNumber::new("[[1,2],3]");
    assert_eq!(format!("{}", number), "[[1,2],3]");
}

#[test]
fn test_construction_3() {
    let number = SnailfishNumber::new("[9,[8,7]]");
    assert_eq!(format!("{}", number), "[9,[8,7]]");
}

#[test]
fn test_construction_4() {
    let number = SnailfishNumber::new("[[1,9],[8,5]]");
    assert_eq!(format!("{}", number), "[[1,9],[8,5]]");
}

#[test]
fn test_construction_5() {
    let number = SnailfishNumber::new("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]");
    assert_eq!(format!("{}", number), "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]");
}

#[test]
fn test_construction_6() {
    let number = SnailfishNumber::new("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]");
    assert_eq!(format!("{}", number), "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]");
}

#[test]
fn test_construction_7() {
    let number = SnailfishNumber::new("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
    assert_eq!(format!("{}", number), "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
}

#[test]
fn test_explosions_1() {
    let mut number = SnailfishNumber::new("[[[[[9,8],1],2],3],4]");
    number.handle_explosion();
    assert_eq!(number, SnailfishNumber::new("[[[[0,9],2],3],4]"));
}

#[test]
fn test_explosions_2() {
    let mut number = SnailfishNumber::new("[7,[6,[5,[4,[3,2]]]]]");
    number.handle_explosion();
    assert_eq!(number, SnailfishNumber::new("[7,[6,[5,[7,0]]]]"));
}

#[test]
fn test_explosions_3() {
    let mut number = SnailfishNumber::new("[[6,[5,[4,[3,2]]]],1]");
    number.handle_explosion();
    assert_eq!(number, SnailfishNumber::new("[[6,[5,[7,0]]],3]"));
}

#[test]
fn test_explosions_4() {
    let mut number = SnailfishNumber::new("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
    number.handle_explosion();
    assert_eq!(number, SnailfishNumber::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));
}

#[test]
fn test_explosions_5() {
    let mut number = SnailfishNumber::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    number.handle_explosion();
    assert_eq!(number, SnailfishNumber::new("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
}

#[test]
fn test_splits_1() {
    let mut number = SnailfishNumber::new("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
    number.handle_explosion();
    number.handle_split();
    number.handle_split();
    assert_eq!(number, SnailfishNumber::new("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"));
}

#[test]
fn test_reduction_1() {
    let mut number = SnailfishNumber::new("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
    number.reduce();
    assert_eq!(number, SnailfishNumber::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
}

#[test]
fn test_addition_1() {
    let number1 = SnailfishNumber::new("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let number2 = SnailfishNumber::new("[1,1]");
    assert_eq!(number1 + number2, SnailfishNumber::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
}

#[test]
fn test_addition_2() {
    let number1 = SnailfishNumber::new("[1,1]");
    let number2 = SnailfishNumber::new("[2,2]");
    let number3 = SnailfishNumber::new("[3,3]");
    let number4 = SnailfishNumber::new("[4,4]");

    assert_eq!(number1 + number2 + number3 + number4,
               SnailfishNumber::new("[[[[1,1],[2,2]],[3,3]],[4,4]]"));
}

#[test]
fn test_addition_3() {
    let number1 = SnailfishNumber::new("[1,1]");
    let number2 = SnailfishNumber::new("[2,2]");
    let number3 = SnailfishNumber::new("[3,3]");
    let number4 = SnailfishNumber::new("[4,4]");
    let number5 = SnailfishNumber::new("[5,5]");

    assert_eq!(number1 + number2 + number3 + number4 + number5,
               SnailfishNumber::new("[[[[3,0],[5,3]],[4,4]],[5,5]]"));
}

#[test]
fn test_addition_4() {
    let number1 = SnailfishNumber::new("[1,1]");
    let number2 = SnailfishNumber::new("[2,2]");
    let number3 = SnailfishNumber::new("[3,3]");
    let number4 = SnailfishNumber::new("[4,4]");
    let number5 = SnailfishNumber::new("[5,5]");
    let number6 = SnailfishNumber::new("[6,6]");

    assert_eq!(number1 + number2 + number3 + number4 + number5 + number6,
               SnailfishNumber::new("[[[[5,0],[7,4]],[5,5]],[6,6]]"));
}

#[test]
fn test_magnitude_1() {
    let number = SnailfishNumber::new("[[9,1],[1,9]]");
    assert_eq!(number.magnitude(), 129);
}

#[test]
fn test_magnitude_2() {
    let number = SnailfishNumber::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    assert_eq!(number.magnitude(), 3488);
}
