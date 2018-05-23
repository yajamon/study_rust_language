// 中の値を、ヴァリアントと呼ぶ
// - データを持たないヴァリアント (unit-like構造体のよう
// - 名前付きデータを持つヴァリアント (構造体のよう
// - 名前無しデータを持つヴァリアント (タプル構造体のよう
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String), // new-type パターンのよう
}
fn main() {
    // enumは一つの型であり、どのヴァリアントの値とマッチしうるものとして扱われる。
    // このことから、直和型(sum type)と呼ばれることもある

    // 各ヴァリアントの名前を使うためには、`::`構文を使う。
    // ヴァリアントの名前は、enum自体の名前によってスコープ化されている。
    // 以下はどちらも上手く動く。
    let x: Message = Message::Move { x: 3, y: 4 };

    enum BoardGameTurn {
        Move { squares: i32 },
        Pass,
    }
    let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };

    // 列挙型の値は、ヴァリアントに関連するデータに加えて、
    // その値自身がどのヴァリアントかという情報を持つ
    // コンパイラはこれを利用して、安全なアクセスを強制する
    //
    // 以下のようには書けない
    // fn process_color_change(msg: Message) {
    //     let Message::ChangeColor(r, g, b) = msg
    // }
    //
    // この制限は match 式でヴァリアントのパターンマッチを行う
    // または、等値性を実装する（トレイトのセクションで説明）

    // コンストラクタを関数のように扱うこともできる
    let m = Message::Write("Hello, world".to_string());

    // これは以下と同じだそう
    fn foo(x: String) -> Message {
        Message::Write(x)
    }
    let x = foo("Hello, world".to_string());

    // これはクロージャのセクションで効いてくるらしい。
    // イテレータと合わせて StringのベクタからMessage::Writeのベクタへ変換できる
    let v = vec!["Hello".to_string(), "World".to_string()];
    let v1: Vec<Message> = v.into_iter().map(Message::Write).collect();
}
