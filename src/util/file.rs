use std::fs::{File, Metadata};
use crate::errors::{DaMieError, rs_error};


pub struct RustFile {
    name: String, // 文件名
    uuid: String, // 唯一标识码
    md5: String,  // 文件哈希
    size: u64, // 初始文件大小
    z_size: u64, //压缩文件大小
    c_time: String, // 创建时间
    owner: String, // 拥有者
    group: String, // 拥有组
    public: String, // 是否公开, 父目录权限优先
    file_type: String,  // 文件类型
    tar: bool,  // 是否压缩, 默认为 tar.gz 压缩
    crypt: bool,   // 是否加密 默认为X25519加密
    child: Vec<String>, // 子文件夹及子目录
    parent: Vec<String> // 父文件夹, 运用于列表方便实现复制功能
}

pub struct Dir {
    name: String, // 文件夹名
    uuid: String, // 唯一标识码
    size: u64,    // 文件夹总大小
    z_size: u64,  // 文件夹压缩文件大小
    c_time: String, // 创建时间
    owner: String, // 拥有者
    public: String, // 分为公开(0), 私密(1), 和强制私密(-1), 强制私密将忽略父目录状态以及覆盖子目录状态
    child: Vec<String>, //子文件夹及子目录
    parent: Vec<String> //父文件夹, 运用于列表方便实现复制功能
}

pub struct Data {

}

pub fn metadata(path: &str) -> Result<Metadata, DaMieError> {
    let mut f = match File::open(path){
        Ok(f) => f,
        Err(_) => return Err(rs_error("open file for metadata"))
    };
    let data = match f.metadata(){
        Ok(d) => d,
        Err(_) => return Err(rs_error("get file metadata"))
    };

}