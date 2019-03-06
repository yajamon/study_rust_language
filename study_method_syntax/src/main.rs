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

fn main() {
    // Rustは `impl` キーワードによって「メソッド呼び出し構文」の機能を提供している

    let c = Circle {
        x: 0.0,
        y: 0.0,
        radius: 2.0,
    };
    println!("{}", c.area());
    let d = c.grow(2.0).area();
    println!("{}", d);
}
