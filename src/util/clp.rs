use std::{string, vec};
use clipboard_win::raw;

// 获取 windows 操作系统, 最近复制的文件列表
pub fn get_copied_files(v: &mut vec::Vec<string::String>) -> usize{
    raw::open();
    let count = raw::get_file_list(v);
    raw::close();
    match count {
        Ok(n) => n,
        Err(..) => 0
    }
}