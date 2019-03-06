struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    // 特別な第一引数 `self` (変形として `&self` `&mut self`)
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    // Circleを返すことにより、メソッドチェーンを実装できる
    fn grow(&self, increment: f64) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            radius: self.radius + increment,
        }
    }
}

// 好きなだけimplブロックを使用できる
impl Circle {
    fn reference(&self) {
        println!("taking self by reference!");
    }
}

impl Circle {
    // selfを引数に取らない関数も定義できる。これを関連関数と呼ぶ。
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }
}

// ユーザがCircleを作成できるようにしつつも、書き換えたいプロパティだけ設定するようにしたい
// Rustにはメソッドのオーバーロードや名前付き引数、可変子引数といった機能がない
// そのかわりにBuilderパターンを採用している
struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}
impl CircleBuilder {
    fn new() -> CircleBuilder {
        CircleBuilder {
            x: 0.0,
            y: 0.0,
            radius: 1.0,
        }
    }
    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }
    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.y = coordinate;
        self
    }
    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }
    fn finalize(&self) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            radius: self.radius,
        }
    }
}

fn main() {
    // Rustは `impl` キーワードによって「メソッド呼び出し構文」の機能を提供している

    // let c = Circle {
    //     x: 0.0,
    //     y: 0.0,
    //     radius: 2.0,
    // };
    // 関連関数は Struct::function() という構文で呼び出される
    let c = Circle::new(0.0, 0.0, 2.0);
    println!("{}", c.area());
    let d = c.grow(2.0).area();
    println!("{}", d);

    // Builderパターンによる書き方はこんなかんじ
    let c = CircleBuilder::new().x(1.0).y(2.0).radius(2.0).finalize();
    println!("area: {}", c.area());
    println!("x: {}", c.x);
    println!("y: {}", c.y);
}
