// ベクタは動的な、また拡張可能な配列
// ジェネリクスによってあらゆる型に対応する
// ベクタは常にヒープアロケーションを行う
fn main() {
    // ベクタの生成
    let v = vec![1, 2, 3, 4, 5];
    // 初期値の繰り返し
    let v1 = vec![0; 10]; // 0 が 10個

    // 特定インデックスへ添字によるアクセス (添字はusize型)
    println!("The third element of v is {}", v[2]);

    // forによるイテレーティング
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("A reference to {}", i);
    }
    for i in &mut v {
        println!("A mutable reference to {}", i);
    }
    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }
}
