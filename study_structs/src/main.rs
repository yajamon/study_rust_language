// 名称は慣例的に始めが大文字のキャメルケース
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // structにまとめることで、以下のように書かなくてよくなる。
    // let origin_x = 0;
    // let origin_y = 0;
    let origin = Point { x: 0, y: 0 };

    // 束縛にミュータビリティを付与することで値を変更できる
    let mut point = Point { x: 0, y: 0 };
    point.x = 5;

    // 新しい束縛によってここから変更できなくなる
    let point = point;

    // // 変更できないのでこれはエラー
    // point.y = 6;

    println!("The origin is at ({}, {})", origin.x, origin.y);

    // アップデート構文
    // 初期化時に他の構造体から値をコピーすることができる
    let mut point = Point3d { x: 0, y: 0, z: 0 };
    point = Point3d { y: 0, ..point };

}
