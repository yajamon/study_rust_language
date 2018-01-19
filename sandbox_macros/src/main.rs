macro_rules! m_vec {
    (hello) => (println!("Hello, world."));
    ($e:expr) => (println!("Hello, marro. matched to string: {}", $e));
}

macro_rules! builder {
    ($src_name:ident => $dest_name:ident {
        $( $attr_name:ident : $attr_type:ty ),*
    }) => {
        struct $dest_name {
            $( $attr_name : $attr_type ),*
        }
        struct $src_name {
            $( $attr_name : $attr_type ),*
        }
    }
}

fn main() {
    m_vec!("hey ruster");
    m_vec!(hello);

    builder!(UserBuilder => User {
        id: i32,
        name: String
    });

    let user = User {
        id: 1,
        name: "Pater".to_string(),
    };

    println!("id: {}, name: {}", user.id, user.name);
}
