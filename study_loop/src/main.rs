fn main() {

    // loop: 無限ループ
    // while trueを使うくらいならこれを使う
    // コンパイルしたら1ループにつき1つ比較用のステップが節約される？
    let mut count = 0;
    loop {
        count = count + 1;
        println!("Loop!! count: {}", count);
        if count >= 10 {
            break;
        }
    }

    // while: よくみるやつ
    let mut done = false;
    let mut count = 10;
    while !done {
        count = count - 1;
        println!("While!! count: {}", count);
        if count <= 0 {
            done = true;
        }
    }

    // for: c言語っぽい書き方はできないよ
    // 特定の回数だけループする時に使う
    // イテレーターぶん回すときに使う
    for v in 0..10 {
        println!("for!! v: {}", v);
    }

    // forで何回目の繰り返しかはどうやって知る？
    // .enumerate()を使う
    for (index, value) in (5..10).enumerate() {
        println!("for!! index: {} and value: {}", index, value);
    }

    // loop label
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; }
            if y % 2 == 0 { continue 'inner; }
            println!("x: {}, y: {}", x, y);
        }
    }
}

