use std::ffi::CString;
use std::fs::{File, Metadata, OpenOptions};
use std::hash::Hash;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::mem;
use std::mem::MaybeUninit;
use std::os::windows::fs::FileExt;
use std::path::Path;
use std::str::from_utf8;
use crate::errors::{DaMieError, rs_error};


pub struct RustFile {
    name: String, // 文件名
    key: String, // 唯一标识码同时作为解压密码
    uuid: String,  // 文件哈希
    size: u64, // 初始文件大小
    z_size: u64, //压缩文件大小
    c_time: String, // 创建时间
    m_time: String, // 修改时间
    public: String, // 是否公开, 父目录权限优先
    file_type: String,  // 文件类型
    tar: bool,  // 是否压缩, 默认为 tar.gz 压缩
    crypt: bool,   // 是否加密
    child: Vec<String>, // 子文件夹及子目录
    parent: Vec<String> // 父文件夹, 运用于列表方便实现复制功能
}

pub struct Dir {
    name: String, // 文件夹名
    uuid: String, // 唯一标识码
    size: u64,    // 文件夹总大小
    z_size: u64,  // 文件夹压缩文件大小
    c_time: String, // 创建时间
    m_time: String, // 修改时间
    public: String, // 分为公开(0), 私密(1), 和强制私密(-1), 强制私密将忽略父目录状态以及覆盖子目录状态
    child: Vec<String>, //子文件夹及子目录
    parent: Vec<String> //父文件夹, 运用于列表方便实现复制功能
}

pub struct Data {

}

pub fn metadata(path: &Path) -> Result<Metadata, DaMieError> {
    let mut f = match File::open(path){
        Ok(f) => f,
        Err(_) => return Err(rs_error("open file for metadata"))
    };
    f.metadata().map_err(|_| rs_error("get file metadata"))
}

pub fn read_file_header(path: &Path) -> Result<Vec<u8>, DaMieError>{
    let mut buf = [0u8; 4];
    let mut file = File::open(path)?;
    file.read_exact(&mut buf);
    Ok(buf.to_vec())
}

pub fn read_file(path: &Path) -> Result<Vec<u8>, DaMieError> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut buffer: Vec<u8> = Vec::new();
    buf_reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn save_file(data: Vec<u8>, path: &Path) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(&data)?;
    Ok(())
}

pub fn to_fake_gz(path: &Path, uuid: &str) -> Result<usize, DaMieError> {
    let header_old = read_file_header(path)?;
    let header:[u8; 4] = [77, 90, 144, 0];
    let mut buf = [0u8; 4];
    let mut f = BufReader::new(File::open(path).unwrap());
    let mut file = OpenOptions::new().write(true).open(path)?;
    while f.read_exact(&mut buf).is_ok() {
        println!("{:?}", buf);
    };
    Ok(file.write(&header)?)
}