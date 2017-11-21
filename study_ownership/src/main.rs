fn main() {
    foo();
    bar();
}

fn foo() {
    let v = vec![1, 2, 3];

    // ムーブセマンティクス
    // - リソースに対して束縛できる変数はひとつだけ
    let v2 = v;
    println!("v2[0] is: {}", v2[0]);
    // // vを利用することはできない
    // println!("v[0] is: {}", v[0]);

    // Copy 型: トレイトという仕組みを使ったCopyというものがある
    // すべてのプリミティブ型はCopyトレイトを実装している
    let val = 1;
    let val2 = val;
    println!("value is: {}", val);

}

fn bar() {
    // 関数の引数、戻り値にmoveを多用すれば、とても煩雑になる

    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    let (v1, v2, answer) = buz(v1, v2);
    println!("{} {} {}", v1[0], v2[0], answer);
}

fn buz(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    (v1, v2, 42)
}

