fn main() {
    // 標準的な変数束縛
    let first = 5;
    println!("first: {}", first);

    // let の左側の式はパターンであって、変数宣言というだけではない
    // こういうこともできる
    let (left, right) = (10, 20);
    println!("left: {} right: {}", left, right);
    // 詳細は別途パターンのセクションで

    // Rustは型を必要とするプログラミング言語だが、型推論を持っている
    // 型を明記してfirstを書くと以下のようになる
    let second: i32 = 5;
    println!("second: {}", second);

    // 変数束縛は、デフォルトでイミュータブルである
    //
    // // よって再代入はできない
    // second =  23; // compile errorj
    // println!("second: {}", second);

    // mutキーワードを付与することで、ミュータブルとすることができる
    let mut mut_value = 10;
    println!("mut value: {}", mut_value);
    mut_value = 25;
    println!("mut value: {}", mut_value);

    // 未使用の変数が存在することは、rustは警告するがコンパイル、実行に影響はない
    let un_initialize_value: i32;
    println!("declare un_initialize_value");
    // // 初期化していない変数の使用を試みるとコンパイルエラー
    // println!("un_initialize_value: {}", un_initialize_value);

    // { } を使用して任意にブロックを切ることができる
    // 変数束縛のスコープは定義されたブロックの範囲内でのみ有効
    let outside = 10;
    {
        let inside = 20;
        println!("outside in block {}", outside);
        println!("inside in block {}", inside);
    }
    println!("outside {}", outside);
    // println!("inside {}", inside); // compile error

    // シャドーイングという仕組みがある
    // 同名の変数束縛を定義すると上書きすることができる
    let shadow_base: i32 = 1;
    {
        println!("shadow_base {}", shadow_base);
        let shadow_base = 21;
        println!("shadow_base {}", shadow_base);
    }
    // ブロックの中でシャドーイングをした場合、スコープを抜けると元の変数束縛が表に出てくる
    println!("shadow_base {}", shadow_base);
    // 同じスコープでシャドーイングするともう元の変数束縛は取り出せないと考えられる。
    // デバッグするときは、トラブルポイントから辿って
    // 最初にletされている箇所の型を信頼すればいいから楽といえば楽なのかな。
    let shadow_base = 42;
    println!("shadow_base {}", shadow_base);

    // ミュータブルをシャドーイングによってイミュータブルにしてしまう例
    let mut integer: i32 = 12;
    integer = 3232;
    let integer = integer;
    // integer = 1212; // compile error
    println!("integer {}", integer);

    // シャドーイングによって別の型を束縛してしまう例
    let number_or_string = 100;
    println!("number_or_string {}", number_or_string);
    let number_or_string = "Changed variable bind";
    println!("number_or_string {}", number_or_string);
}
