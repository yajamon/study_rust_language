fn main() {
    // パターンはRustにおいて極めて一般的
    let x = 1;

    // リテラルに対しては直接マッチさせられる
    // _ は任意のケースとして振る舞う
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
