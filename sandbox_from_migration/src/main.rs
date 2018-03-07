fn main() {
    let c = get_config(1);
    println!("config: {:?}", c);
    let c2: Configs::V2 = get_config(2);
}

fn get_config(version: u8) -> ConfigList {
    match version {
        1 => ConfigList::V01 { version: version },
        2 => ConfigList::V02 {
            version: version,
            a: "AAA".to_string(),
        },
        _ => panic!("undefined version: {}", version),
    }
}

#[derive(Debug)]
enum ConfigList {
    V01 { version: u8 },
    V02 { version: u8, a: String },
}

struct ConfigV01 {
    version: u8,
}
struct ConfigV02 {
    version: u8,
    a: String,
}
