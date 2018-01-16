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
}
