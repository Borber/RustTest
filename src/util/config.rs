use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use crate::errors::{DaMieError, rs_error};

// TODO 定义配置

#[derive(Deserialize)]
pub struct RustConfig {
    mlar: MlarConfig
}

#[derive(Deserialize)]
pub struct MlarConfig {
    path: String
}

impl std::fmt::Display for RustConfig {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, indoc::indoc!(r"
          Config:
          [mlar]
          path = {}
          "),
               self.mlar.path)
    }
}

// TODO 读取配置

pub fn get_config(name: &String) -> Result<RustConfig, DaMieError>{
    let mut file = match File::open(name) {
        Ok(f) => f,
        Err(_) => {
            return Err(rs_error("open config file"));
        }
    };
    let mut str_val = String::new();
    if file.read_to_string(&mut str_val).is_err() {
        return Err(rs_error("open config file"));
    }
    toml::from_str(&str_val)
        .map_err(|_| rs_error("get config from str"))
}

// TODO 初始化配置



