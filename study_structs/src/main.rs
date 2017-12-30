// 名称は慣例的に始めが大文字のキャメルケース
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // structにまとめることで、以下のように書かなくてよくなる。
    // let origin_x = 0;
    // let origin_y = 0;
    let origin = Point {x: 0, y: 0};

    println!("The origin is at ({}, {})", origin.x, origin.y);
}
