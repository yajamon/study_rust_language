fn main() {
    let c = get_config(1);
    println!("config: {:?}", c);
    let c2: ConfigList = get_config(2);
    println!("config: {:?}", c);
}

fn get_config(version: u8) -> ConfigList {
    match version {
        1 => ConfigList::V01(ConfigV01 { version: version }),
        2 => ConfigList::V02(ConfigV02 {
            version: version,
            a: "AAA".to_string(),
        }),
        _ => panic!("undefined version: {}", version),
    }
}

#[derive(Debug)]
enum ConfigList {
    V01(ConfigV01),
    V02(ConfigV02),
}

#[derive(Debug)]
struct ConfigV02 {
    version: u8,
    a: String,
}
// 一つ前のバージョンをmigrate
impl From<ConfigV01> for ConfigV02 {
    fn from(config: ConfigV01) -> Self {
        ConfigV02 {
            version: config.version,
            a: "Migrated".to_string(),
        }
    }
}

#[derive(Debug)]
struct ConfigV01 {
    version: u8,
}
