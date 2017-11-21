fn main() {
    bar();
}

fn bar() {
    // 関数の引数、戻り値にmoveを多用すれば、とても煩雑になる

    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    let answer = buz(&v1, &v2);
    println!("{} {} {}", v1[0], v2[0], answer);
}

fn buz(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    42
}

