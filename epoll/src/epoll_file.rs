//! epoll 类型文件

use base_file::File;
use lock::Mutex;
use alloc::sync::Arc;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;
use crate::{EpollEvent, EpollCtl, EpollErrorNo};

/// 用作 epoll 的文件
pub struct EpollFile {
    inner: Arc<Mutex<EpollFileInner>>,
}

/// epoll 内部可变部分
struct EpollFileInner {
    /// 监控的所有文件(fd)。key 不用 Arc<dyn File> 只是因为不好针对 map 做
    interest_list: BTreeMap<i32, EpollEvent>,
    /// 已经相应事件的文件(fd)
    _ready_list: BTreeSet<i32>,
}

impl EpollFile {
    /// 新建一个 epoll 文件
    pub fn new() -> Self {
        Self {
            inner : Arc::new(Mutex::new(EpollFileInner {
                interest_list: BTreeMap::new(),
                _ready_list: BTreeSet::new(),
            }))
        }
    }
    /// 获取另一份 epoll 文件。即使这个文件被删除，只要 fd_manager 里还存有一份，
    /// 内部 inner 上的 Arc 就不会不归零，数据就不会被删除
    pub fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone()
        }
    }
    /// 进行控制操作，如成功则返回 Ok(())，否则返回对应的错误编号
    pub fn epoll_ctl(&self, op: EpollCtl, fd: i32, event: EpollEvent) -> Result<(), EpollErrorNo> {
        let list = &mut self.inner.lock().interest_list;
        match op {
            EpollCtl::ADD => {
                if list.contains_key(&fd) {
                    return Err(EpollErrorNo::EEXIST);
                } else {
                    list.insert(fd, event);
                }
            },
            EpollCtl::MOD => {
                if list.contains_key(&fd) {
                    // 根据 BTreeMap 的语义，这里的 insert 相当于把原来的值替换掉
                    list.insert(fd, event);
                } else {
                    return Err(EpollErrorNo::ENOENT);
                }
            },
            EpollCtl::DEL => {
                if list.remove(&fd).is_none() {
                    return Err(EpollErrorNo::ENOENT);
                }
            }
        }
        Ok(())
    }
    /// 获取 interest_list 中的所有 epoll 事件
    pub fn get_epoll_events(&self) -> Vec<EpollEvent> {
        let interest = &self.inner.lock().interest_list;
        let mut events: Vec<EpollEvent> = Vec::new();
        for (fd, evt) in interest {
            let mut nevt = *evt;
            if *fd as u64 != nevt.data {
                // warn!("fd: {} is not in Event: {:?}", fd, evt);
                nevt.data = *fd as u64;
            }
            events.push(nevt);
        }
        return events;
    }
}

impl File for EpollFile {
    /// epoll 文件不可直接读
    fn read(&self, _buf: &mut [u8]) -> Option<usize> {
        None
    }
    /// epoll 文件不可直接写
    fn write(&self, _buf: &[u8]) -> Option<usize> {
        None
    }
}
