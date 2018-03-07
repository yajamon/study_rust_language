fn main() {
    let c = get_config(1);
    println!("config: {:?}", c);
    if let ConfigList::V01(config) = c {
        let c = ConfigV02::from(config);
        println!("config: {:?}", c);
    }
}

type Config = ConfigV03;

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

fn migrate(config: ConfigList) -> Config {
    let mut config = config;
    loop {
        config = match config {
            ConfigList::V01(c) => ConfigList::V02(ConfigV02::from(c)),
            ConfigList::V02(c) => ConfigList::V03(ConfigV03::from(c)),
            ConfigList::V03(c) => return c,
        };
    }
}

#[derive(Debug)]
enum ConfigList {
    V01(ConfigV01),
    V02(ConfigV02),
    V03(ConfigV03),
}

#[derive(Debug)]
struct ConfigV03 {
    version: u8,
    a: String,
    b: String,
}
impl From<ConfigV02> for ConfigV03 {
    fn from(config: ConfigV02) -> Self {
        ConfigV03 {
            version: config.version,
            a: config.a.clone(),
            b: format!("String '{}' by A", config.a),
        }
    }
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
