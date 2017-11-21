fn main() {
    // bool: boolean
    let flag: bool = true;
    if flag {
        println!("flag is true");
    } else {
        println!("flag is false");
    }

    // char: 1つのunicodeスカラ値 (4byte)
    let x = 'x';
    // let two_hearts = ''; // 絵文字とかいける

    // numeric: 数値型
    // i8 i16 i32 i64
    // u8 u16 u32 u64
    // isize
    // usize
    // f32 f64

    // 符号 i u
    // i: 符号あり 整数
    // u: 符号なし 整数
    let iByte: i8 = 127;
    let uByte: u8 = 255;
    println!("iByte {}", iByte);
    println!("uByte {}", uByte);
    let iByteOverflow: i8 = 128; // 即値を与える場合はコンパイルエラーにならない(警告はされる)
    let uByteOverflow: u8 = 256; // 即値を与える場合はコンパイルエラーにならない(警告はされる)
    println!("overflow iByte {}", iByteOverflow);
    println!("overflow uByte {}", uByteOverflow);
    // 固定長 符号の後の数字はビット長を表す
    // 可変長 size
    // f: 浮動小数点型

    // 配列
    // [T; N] という型を持つ
    // Tはジェネリクスの項で。Nは配列の長さ。
    let array = [1, 2, 3]; // array: [i32: 3]
    let mut mutArray = [1, 2, 3]; // mutArray: [i32: 3]

    // 同じ値で初期化するための省略表現がある。
    // 0で20個の要素を埋める
    let zeroFill = [0; 20];

    // len(): 配列の要素数を返す。
    println!("array has {} elements", array.len());
}
