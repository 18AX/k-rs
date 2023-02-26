use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
pub enum NodeKind {
    File(FileInfo),
    Directory(Directory),
}

pub struct Node {
    pub name: String,
    pub kind: NodeKind,
}

pub enum MoveCursorKind {
    Start,
    Current,
    End,
}

pub enum FileError {
    NotImplemented,
}

pub trait FileOperation {
    fn close(&self) -> Result<usize, FileError>;
    fn write(&self, data: Vec<u8>) -> Result<usize, FileError>;
    fn read(&self, data: Vec<u8>) -> Result<usize, FileError>;
    fn flush(&self) -> Result<(), FileError>;
    fn rename(&self, name: &str) -> Result<(), FileError>;
    fn move_cursor(&self, kind: MoveCursorKind, offset: usize) -> Result<usize, FileError>;
    fn cursor(&self) -> Result<usize, FileError>;
}

pub struct File {
    info: FileInfo,
    size: usize,
    cursor: usize,
    operation: Box<dyn FileOperation>,
}

pub trait FileInfoOperation {
    fn open(&self) -> Result<File, FileError>;
}

pub struct FileInfo {
    pub operation: Box<dyn FileInfoOperation>,
}

pub enum DirectoryError {
    NotImplemented,
    NodeAlreadyExist,
}

pub trait DirectoryOperation {
    fn mount(&self) -> Result<(), DirectoryError>;
    fn list_nodes(&self) -> Result<Vec<Arc<Node>>, DirectoryError>;
    fn find_node(&self, name: &str) -> Option<Arc<Node>>;
    fn add_node(&mut self, node: Node) -> Result<(), DirectoryError>;
}

pub struct Directory {
    pub operation: Box<dyn DirectoryOperation>,
}

pub struct Vfs {
    root: Node,
}

impl Vfs {
    pub const fn new(root: Node) -> Self {
        Vfs { root }
    }
}

static mut ROOT: Option<Vfs> = Option::None;

pub fn open(path: &str) -> Result<Arc<File>, FileError> {
    Err(FileError::NotImplemented)
}

pub fn set_root(vfs: Vfs) {
    unsafe {
        ROOT = Some(vfs);
    }
}
