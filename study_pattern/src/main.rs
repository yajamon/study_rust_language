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

    // パターンの落とし穴。パターンはシャドーイングを行う
    let x = 'x';
    let c = 'c';

    match c {
        x => println!("x: {}, c: {}", x, c),
    }
    println!("x: {}", x);
    // 以下のように出力される
    // x: c, c: c
    // x: x

    // `x =>`は値をパターンにマッチさせ、`match`のスコープ内で有効な`x`という名前の束縛を導入する。
    // すでに`x`という束縛が存在するため、新たに導入した`x`は、古い`x`をシャドーイングする。
    // という言い方ができる。

    // 複式パターン
    // `|`を使うと、複数のパターンにマッチさせることができる
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 分配拘束
    // structのような複合データ構造が存在するとき、パターン内で分解できる
    struct Point {
        x: i32,
        y: i32,
    }
    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, y } => println!("({}, {})", x, y),
    }
}
