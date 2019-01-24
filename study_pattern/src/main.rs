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

    // 分配束縛
    // structのような複合データ構造が存在するとき、パターン内で分解できる
    struct Point {
        x: i32,
        y: i32,
    }
    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, y } => println!("({}, {})", x, y),
    }
    // 別名をつけるには: をつける方法がある
    match origin {
        Point { x: x1, y: y1 } => println!("({}, {})", x1, y1),
    }
    // 一部の名前にだけ興味がある場合は、必要なものだけを束縛できる
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
    match origin {
        Point { y, .. } => println!("y is {}", y),
    }

    // 束縛の無視
    // パターン内の型や値を無視するために `_` を使うことができる
    let some_value: Result<u32, ()> = Ok(123);
    match some_value {
        Ok(value) => println!("got a value: {}", value),
        Err(_) => println!("an error occurred"),
    }
    // タプルでも
    fn coordinate() -> (i32, i32, i32) {
        (1, 10, 100)
    }
    let (x, _, z) = coordinate();
    println!("x: {} z: {}", x, z);

    // ref, ref mutキーワードで参照を取得できる
    let p = Point { x: 1, y: 3 };
    match p {
        ref r => println!("print x: {}, y: {}", r.x, r.y),
    }
    // pがmoveしていない
    println!("print x: {}, y: {}", p.x, p.y);
    // mutableな参照も同様に書ける
    let mut p = Point { x: 2, y: 4 };
    match p {
        ref mut r => println!("print x: {}, y: {}", r.x, r.y),
    }

    // 範囲
    // `...` で値の範囲をマッチさせることができる
    let x = 1;
    match x {
        1...5 => println!("one through five"),
        _ => println!("anything"),
    }
    // char型でも使われる
    let x = '🎁';
    match x {
        'a'...'j' => println!("early letter"),
        'k'...'z' => println!("late letter"),
        _ => println!("something else"),
    }

    // 束縛
    // `@`で値に名前を束縛できる
    let x = 1;
    match x {
        e @ 1...5 => println!("got a range element: {}", e),
        _ => println!("anything"),
    }
    // 複雑なマッチング
    #[derive(Debug)]
    struct Person {
        name: Option<String>,
    }
    let name = "Steve".to_string();
    let mut x: Option<Person> = Some(Person { name: Some(name) });
    match x {
        Some(Person {
            name: ref a @ Some(_),
            ..
        }) => println!("{:?}", a),
        _ => {}
    }
    // @ と | を組み合わせるパターン
    // それぞれのパターンで同じ名前で束縛する必要がある
    let x = 5;
    match x {
        e @ 1...5 | e @ 8...10 => println!("got a range element: {}", e),
        _ => println!("anything"),
    }

    // ガード
    // if を使うことでパターン中にガードができる
    enum OptionalInt {
        Value(i32),
        Missing,
    }
    let x = OptionalInt::Value(5);
    match x {
        OptionalInt::Value(i) if i > 5 => println!("Got an bigger than five!"),
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck."),
    }
    // 複式パターンでifを使うと、ifは | の両側に適用される
    let x = 4;
    let y = false;
    match x {
        4 | 5 if y => println!("yes"),
        _ => println!("no"),
    }

    // パターンは強力で、上記それぞれを混ぜて使うこともできる。
    // 必要に応じてマッチさせよう。
}
