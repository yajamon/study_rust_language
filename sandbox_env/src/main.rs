use std::env;

fn main() {
    // env::varは引数のkeyを元に環境変数を模索する
    let key = "HOME";
    match env::var(key) {
        Ok(val) => println!("{}: {:?}", key, val),
        Err(e) => println!("取得できませんでした {}: {}", key, e),
    }

    // 引数を振り回すこともできる
    println!("引数の一覧");
    for argument in env::args() {
        println!("{}", argument);
    }

    // 環境変数も振り回すこともできる
    println!("環境変数の一覧");
    for (key, val) in env::vars() {
        println!("{}: {:?}", key, val);
    }
}
