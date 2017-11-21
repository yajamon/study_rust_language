fn main() {
    nothing_argument();
    print_number(10);
    print_sum(10, 5);
    print_number(add_one(19));

    // 関数ポインタ
    let f: fn(i32) -> i32 = add_one;
    print_number(f(12));
}

fn nothing_argument(){
}

// 引数の型情報は必須
fn print_number(x: i32){
    println!("x is {}", x);
}

fn print_sum(x: i32, y: i32){
    println!("x is {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1
}
