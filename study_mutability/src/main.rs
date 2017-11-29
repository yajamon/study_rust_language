use std::sync::Arc;

fn main() {
    // Error
    // let x = 5;
    // x = 6;

    // Valid
    let mut x = 5;
    x = 6;
    // 変数束縛が何を指すか、が変更できる

    // 束縛が指す先を変更する場合は、ミュータブル参照を使う
    let y = &mut x;
    *y = 8;

    // mut はパターンの一部になれる
    let (mut first, second) = (1, 2);

    // 外側のミュータビリティ
    let arc = Arc::new(5);
    let cloned = arc.clone();

}
