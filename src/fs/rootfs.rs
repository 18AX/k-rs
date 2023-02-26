use super::vfs::{DirectoryError, DirectoryOperation, Node};
use alloc::{sync::Arc, vec::Vec};

pub struct RootDir {
    nodes: Vec<Arc<Node>>,
}

impl RootDir {
    pub const fn new() -> Self {
        RootDir { nodes: Vec::new() }
    }
}

impl DirectoryOperation for RootDir {
    fn mount(&self) -> Result<(), super::vfs::DirectoryError> {
        // Rootfs is already mounted
        Err(DirectoryError::NotImplemented)
    }

    fn list_nodes(&self) -> Result<Vec<Arc<Node>>, super::vfs::DirectoryError> {
        Ok(self.nodes.clone())
    }

    fn find_node(&self, name: &str) -> Option<Arc<Node>> {
        self.nodes.iter().find(|x| x.name.eq(name)).cloned()
    }

    fn add_node(&mut self, node: Node) -> Result<(), DirectoryError> {
        match self.find_node(&node.name) {
            Some(_) => Err(DirectoryError::NodeAlreadyExist),
            None => {
                self.nodes.push(Arc::new(node));
                Ok(())
            }
        }
    }
}
