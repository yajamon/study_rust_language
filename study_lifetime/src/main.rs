
// 黙示的ライフタイム
fn foo(x: &i32) {
}

// 明示的ライフタイム
fn bar<'a>(x: &'a i32) {
    // 'a は「ライフタイムa」と読む
    // <>はご存知Generics
    // 関数はGeneric Parameterを持つことができる
    // ライフタイムはその一種
}

// ２つの参照引数がある場合
fn baz<'b, 'c>(x: &'b i32, y: &'c i32) {
}

// &mut参照が欲しい（先の値が変更可能な参照が欲しい
fn fee<'d>(x: &'d mut i32) {
    // &mut i32 と &'a mut i32は同じもの
}

// 参照を含むstruct使う時、明示的なライフタイム宣言が必要になるらしい
struct Foo<'e> {
    x: &'e i32,
}

impl<'e> Foo<'e> {
    // implでもライフタイムの宣言が必要
    fn x(&self) -> &'e i32 { self.x }
}

fn main() {
    let y = &5;
    let f = Foo {x: y};

    println!("x is: {}", f.x());
}

fn x_or_f <'f>(x: &'f str, y: &'f str) -> &'f str {
    x
}
