fn main() {
    let x = 5;
    // 与えられた式が一致するパターンの式を実行する。
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        // matchは網羅性検査を行う。アンダースコアはその他の値すべてを受け入れる。
        _ => println!("something else"),
    }
}
