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

    // matchは式でもある。つまり、下記のようなことができる。
    let number = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        _ => "something else",
    };
    println!("{}", number);

    process_mesasge(Message::Quit);
    process_mesasge(Message::ChangeColor(0, 127, 255));
    process_mesasge(Message::Move { x: 2, y: 5 });
    process_mesasge(Message::Write("text".to_string()));
}

// 列挙型に対するmatchは網羅性検査と合わさって非常に重要
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn quit() {
    println!("quit");
}
fn change_color(r: i32, g: i32, b: i32) {
    println!("r:{} g:{} b:{}", r, g, b);
}
fn move_cursor(x: i32, y: i32) {
    println!("move to x:{} y:{}", x, y);
}

fn process_mesasge(msg: Message) {
    // matchは網羅性検査を行う
    // 列挙型のすべてのバリアントに対してマッチするパターンがあることを要求する
    // 一つでもマッチしないバリアントを残している場合、_を使わなければ
    // コンパイルエラーが発生する
    match msg {
        Message::Quit => quit(),
        Message::ChangeColor(r, g, b) => change_color(r, g, b),
        Message::Move { x, y } => move_cursor(x, y),
        Message::Write(s) => println!("{}", s),
    }
}
