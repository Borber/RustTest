use std::{ string, vec};
use clipboard_win::{raw, SysResult};
use crate::errors::{DaMieError, rs_error};

// 获取 windows 操作系统, 最近复制的文件列表
pub fn get_copied_files(v: &mut vec::Vec<string::String>) -> Result<usize, DaMieError> {
    get_copied_files_by_clipboard_win(v)
        .map_err(|_| rs_error("Failed to get clipboard"))
}

pub fn get_copied_files_by_clipboard_win(v: &mut vec::Vec<string::String>) -> SysResult<usize> {
    raw::open()?;
    let count = raw::get_file_list(v)?;
    raw::close()?;
    Ok(count)
}