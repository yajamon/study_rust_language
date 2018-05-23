fn main() {
    let c = get_config(1);
    println!("config: {:?}", c);
    let c2: Config = c.migrate();
    println!("config: {:?}", c2);
}

type Config = ConfigV03;

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

#[derive(Debug, Clone)]
enum ConfigList {
    V01(ConfigV01),
    V02(ConfigV02),
    V03(ConfigV03),
}

impl ConfigList {
    fn migrate(&self) -> Config {
        let mut config: ConfigList = self.clone();
        loop {
            config = match config {
                ConfigList::V01(c) => ConfigList::V02(c.into()),
                ConfigList::V02(c) => ConfigList::V03(c.into()),
                ConfigList::V03(c) => return c,
            }
        }
    }
}

#[derive(Debug, Clone)]
struct ConfigV01 {
    version: u8,
    a: String,
}
#[derive(Debug, Clone)]
struct ConfigV02 {
    version: u8,
    a: String,
    b: String,
}

impl From<ConfigV01> for ConfigV02 {
    fn from(c: ConfigV01) -> Self {
        ConfigV02 {
            version: 2,
            a: c.a.clone(),
            b: "migration".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct ConfigV03 {
    version: u8,
    a: String,
    b: String,
    c: bool,
}

impl From<ConfigV02> for ConfigV03 {
    fn from(c: ConfigV02) -> Self {
        ConfigV03 {
            version: 3,
            a: c.a.clone(),
            b: c.b.clone(),
            c: true,
        }
    }
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
