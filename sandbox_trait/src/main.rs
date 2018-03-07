macro_rules! accessor {
    (type : $type:ty {get=$getter:ident} ) => {
        fn $getter(&self) -> $type ;
    };
    (type : $type:ty {set=$setter:ident} ) => {
        fn $setter(&mut self, value:$type) ;
    };
    (type : $type:ty {get=$getter:ident, set=$setter:ident} ) => {
        accessor!(type:$type {get=$getter});
        accessor!(type:$type {set=$setter});
    };
}
macro_rules! accessor_impl {
    ($name:ident : $type:ty {get=$getter:ident} ) => {
        fn $getter(&self) -> $type {
            self.$name
        }
    };
    ($name:ident : $type:ty {set=$setter:ident} ) => {
        fn $setter(&mut self, value: $type) {
            self.$name = value;
        }
    };
    ($name:ident : $type:ty {get=$getter:ident, set=$setter:ident} ) => {
        accessor_impl!($name:$type {get=$getter});
        accessor_impl!($name:$type {set=$setter});
    };
}

trait Horizontal {
    accessor!(type:i64 {get=get_x, set=set_x});
    fn count_up(&mut self) -> i64 {
        let next = self.get_x() + 1;
        self.set_x(next);
        self.get_x()
    }
}
struct Point {
    x: i64,
}

impl Horizontal for Point {
    accessor_impl!(x:i64 {get = get_x, set = set_x});
}

fn main() {
    println!("Hello, world!");
    let mut p = Point { x: 0 };
    p.x += 1;
    p.count_up();
    println!("x: {}", p.x);
}
