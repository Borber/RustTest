use std::collections::HashMap;
use serde::Deserialize;

// TODO 定义配置

#[derive(Deserialize)]
struct Config {
    mlar: MlarConfig
}

#[derive(Deserialize)]
struct MlarConfig {
    path: String
}

impl std::fmt::Display for Config {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, indoc::indoc!(r"
          Config:
          [mlar]
          path = {}
          "),
               self.mlar.path)
    }
}
// TODO 保存配置


// TODO 读取配置

pub fn get_config(name: &String){
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name(name)).unwrap();
    ;
    println!("{}", settings.try_into::<Config>().unwrap())
}

// TODO 更新配置

// TODO 输出配置

// TODO 初始化配置



