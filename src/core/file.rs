use super::{
    ffi, error, vars,
    Friend
};
pub use super::ffi::{
    TOX_FILE_KIND as FileKind,
    TOX_FILE_CONTROL as FileControl
};


#[derive(Clone, Debug)]
pub struct File<T: FileManage> {
    pub target: T,
    pub number: u32,
}

impl<T: FileManage> File<T> {
    pub fn from(target: T, number: u32) -> File<T> {
        File { target: target, number: number }
    }
}


pub trait FileOperate {
    fn control(&self, control: FileControl) -> Result<(), error::FileControlErr>;
    fn send(&self, position: u64, data: &[u8]) -> Result<(), error::FileChunkSendErr>;
    fn seek(&self, position: u64) -> Result<(), error::FileSeekErr>;
    fn get_id(&self) -> Result<Vec<u8>, error::FileGetErr>;
}

impl FileOperate for File<Friend> {
    fn control(&self, control: FileControl) -> Result<(), error::FileControlErr> {
        out!( bool
            err,
            ffi::tox_file_control(
                self.target.core,
                self.target.number,
                self.number,
                control,
                &mut err
            )
        )
    }

    fn send(&self, position: u64, data: &[u8]) -> Result<(), error::FileChunkSendErr> {
        out!( bool
            err,
            ffi::tox_file_send_chunk(
                self.target.core,
                self.target.number,
                self.number,
                position,
                data.as_ptr(),
                data.len(),
                &mut err
            )
        )
    }

    fn seek(&self, position: u64) -> Result<(), error::FileSeekErr> {
        out!( bool
            err,
            ffi::tox_file_seek(
                self.target.core,
                self.target.number,
                self.number,
                position,
                &mut err
            )
        )
    }

    fn get_id(&self) -> Result<Vec<u8>, error::FileGetErr> {
        out!( out
            out <- vec_with!(vars::TOX_FILE_ID_LENGTH),
            err,
            ffi::tox_file_get_file_id(
                self.target.core,
                self.target.number,
                self.number,
                out.as_mut_ptr(),
                &mut err
            )
        )
    }
}


pub trait FileManage {
    fn transmission<F: AsRef<[u8]>>(&self, kind: FileKind, filename: F, filesize: u64, fileid: Option<&[u8]>) -> Result<File<Friend>, error::FileSendErr>;
}

impl FileManage for Friend {
    fn transmission<F: AsRef<[u8]>>(&self, kind: FileKind, filename: F, filesize: u64, fileid: Option<&[u8]>) -> Result<File<Friend>, error::FileSendErr> {
        let filename = filename.as_ref();
        out!( num
            err,
            ffi::tox_file_send(
                self.core,
                self.number,
                kind as ::libc::uint32_t,
                filesize,
                fileid.map(|i| i.as_ptr())
                    .unwrap_or_else(|| ::std::ptr::null()),
                filename.as_ptr(),
                filename.len(),
                &mut err
            )
        ).map(|n| File::from(self.clone(), n))
    }
}


/// Hash Function.
///
/// ```
/// use tox::core::file::hash;
/// assert_eq!(
///     vec![214, 13, 96, 148, 15, 14, 220, 97, 202, 4, 236, 119, 84, 119, 34, 35, 79, 41, 253, 246, 129, 37, 135, 195, 108, 215, 84, 158, 115, 200, 74, 32],
///     hash(b"tox-rs")
/// )
/// ```
pub fn hash(data: &[u8]) -> Vec<u8> {
    out!(get
        out <- vec_with!(vars::TOX_HASH_LENGTH),
        ffi::tox_hash(out.as_mut_ptr(), data.as_ptr(), data.len())
    )
}
