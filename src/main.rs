use fuser::{mount2, FileAttr, FileType, Filesystem, MountOption};
use generate_config::generate_config;
use libc::ENOENT;
use std::{
    fs::{remove_file, File},
    time::{Duration, UNIX_EPOCH},
};
mod generate_config;

const MOUNT_FILE: &str = "/etc/keyd/chromebook.conf";

fn get_file_attr() -> FileAttr {
    FileAttr {
        ino: 1,
        size: generate_config().len() as u64,
        blocks: 1,
        atime: UNIX_EPOCH, // 1970-01-01 00:00:00
        mtime: UNIX_EPOCH,
        ctime: UNIX_EPOCH,
        crtime: UNIX_EPOCH,
        kind: FileType::RegularFile,
        perm: 0o644,
        nlink: 1,
        uid: 501,
        gid: 20,
        rdev: 0,
        flags: 0,
        blksize: 512,
    }
}

struct MyFS {}
impl Filesystem for MyFS {
    fn read(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        _fh: u64,
        offset: i64,
        size: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        reply: fuser::ReplyData,
    ) {
        if ino == 1 {
            let config = generate_config();
            let end = ((offset + (size as i64)) as usize).min(config.len());
            reply.data(&config.as_bytes()[offset as usize..end]);
        } else {
            reply.error(ENOENT);
        }
    }

    fn getattr(&mut self, _req: &fuser::Request, ino: u64, reply: fuser::ReplyAttr) {
        match ino {
            1 => reply.attr(&Duration::from_nanos(0), &get_file_attr()),
            _ => reply.error(ENOENT),
        }
    }

    fn lookup(
        &mut self,
        _req: &fuser::Request<'_>,
        _parent: u64,
        _name: &std::ffi::OsStr,
        reply: fuser::ReplyEntry,
    ) {
        reply.entry(&Duration::from_nanos(0), &get_file_attr(), 0);
    }
}

fn main() {
    let fs = MyFS {};
    File::create(MOUNT_FILE).unwrap();
    mount2(
        fs,
        MOUNT_FILE,
        &[
            MountOption::RO,
            MountOption::AllowOther,
            MountOption::AutoUnmount,
        ],
    )
    .unwrap();
    remove_file(MOUNT_FILE).unwrap();
}
