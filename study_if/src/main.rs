fn main() {
    // standard
    let is_true = true;
    let is_false = false;
    if is_true {
        println!("condition is true");
    } else if is_false {
        println!("noop");
    } else {
        println!("condition is false");
    }

    // if は 式 であるため、値を返すことができる
    let y = if is_false == false { 10 } else { 15 };
    println!("y: {}", y);

    // // elseの無いif式が返すものは常に空のタプルであることが求められる
    // // これは、安全に運用するには空のタプルしか選択肢がないため
    // // また、let if式の場合、その値を再利用することがほとんどのため
    // // コンパイラが異常を検知しやすい副作用がある
    // let tuple = if is_true { 10 };
    // println!("tuple: {}", tuple);

}
