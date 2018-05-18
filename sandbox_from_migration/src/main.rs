fn main() {
    let c = get_config(1);
    println!("config: {:?}", c);
    let c2: Config = migrate(c);
    println!("config: {:?}", c2);
}

type Config = ConfigV02;

fn get_config(version: u8) -> ConfigList {
    match version {
        1 => ConfigList::V01(ConfigV01 {
            version: version,
            a: "AAA".to_string(),
        }),
        2 => ConfigList::V02(ConfigV02 {
            version: version,
            a: "ABABAB".to_string(),
            b: "CDCDCD".to_string(),
        }),
        _ => panic!("undefined version: {}", version),
    }
}

fn migrate(config: ConfigList) -> Config {
    let mut config = config;
    loop {
        config = match config {
            ConfigList::V01(c) => ConfigList::V02(c.into()),
            ConfigList::V02(c) => return c,
        }
    }
}

#[derive(Debug)]
enum ConfigList {
    V01(ConfigV01),
    V02(ConfigV02),
}

#[derive(Debug)]
struct ConfigV01 {
    version: u8,
    a: String,
}
#[derive(Debug)]
struct ConfigV02 {
    version: u8,
    a: String,
    b: String,
}

impl From<ConfigV01> for ConfigV02 {
    fn from(c: ConfigV01) -> ConfigV02 {
        ConfigV02 {
            version: 2,
            a: c.a.clone(),
            b: "migration".to_string(),
        }
    }
}
