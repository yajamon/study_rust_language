macro_rules! m_vec {
    (hello) => (println!("Hello, world."));
    ($e:expr) => (println!("Hello, marro. matched to string: {}", $e));
}

macro_rules! builder {
    ($src_name:ident => $dest_name:ident {
        $( $attr_name:ident : $attr_type:ty = $attr_default:expr ),*
    }) => {
        struct $dest_name {
            $( $attr_name : $attr_type ),*
        }

        struct $src_name {
            $( $attr_name : $attr_type ),*
        }
        impl $src_name {
            pub fn new() -> $src_name {
                $src_name {
                    $( $attr_name : $attr_default ),*
                }
            }

            pub fn build(&self) -> $dest_name {
                $dest_name {
                    $($attr_name : self.$attr_name.clone()),*
                }
            }
        }
    }
}

fn main() {
    m_vec!("hey ruster");
    m_vec!(hello);

    builder!(UserBuilder => User {
        id: i32 = 0,
        name: String = "hoge".to_string()
    });

    let user = UserBuilder::new().build();
    println!("id: {}, name: {}", user.id, user.name);
}
