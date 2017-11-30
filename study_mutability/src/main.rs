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
    // ArcはAtomicReferenceCountedの略称
    // すなわち、clone()を呼び出す時に参照カウントを更新する必要がある
    // まぁとにかく、Arc<T>.clone()は&Tを返す

    // 内側のミュータビリティ
    let cell = RefCell::new(42);
    let mut_cell = cell.borrow_mut();  // 内側の値に対する &mut を配るらしい
    let mut_cell2 = cell.borrow_mut(); // 複数の &mut 参照を配ると panic! するらしい

}
