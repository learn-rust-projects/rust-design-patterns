// Composite Pattern applied to a file system in Rust
// This is a design pattern demonstration where Directory and File are both FileSystemNodes

// Trait that defines common operations
pub trait FileSystemNode {
    fn path(&self) -> &str;
    fn count_files(&self) -> usize;
    fn count_file_size(&self) -> u64;
}

// File node: leaf in the composite structure
pub struct FileNode {
    path: String,
}

impl FileNode {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

impl FileSystemNode for FileNode {
    fn path(&self) -> &str {
        &self.path
    }

    fn count_files(&self) -> usize {
        1
    }

    fn count_file_size(&self) -> u64 {
        // std::fs::metadata(&self.path)
        //     .map(|meta| meta.len())
        //     .unwrap_or(0)
        20
    }
}

// Directory node: composite that can contain children
pub struct DirectoryNode {
    path: String,
    sub_nodes: Vec<Box<dyn FileSystemNode>>,
}

impl DirectoryNode {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            sub_nodes: vec![],
        }
    }

    pub fn add(&mut self, node: Box<dyn FileSystemNode>) {
        self.sub_nodes.push(node);
    }

    pub fn remove(&mut self, path: &str) {
        self.sub_nodes.retain(|node| node.path() != path);
    }
}

impl FileSystemNode for DirectoryNode {
    fn path(&self) -> &str {
        &self.path
    }

    fn count_files(&self) -> usize {
        self.sub_nodes.iter().map(|n| n.count_files()).sum()
    }

    fn count_file_size(&self) -> u64 {
        self.sub_nodes.iter().map(|n| n.count_file_size()).sum()
    }
}

// Demo usage
fn main() {
    // ├── wz/
    // │   ├── a.txt
    // │   ├── movies/
    // │       ├── c.avi
    // Build the file system tree
    let mut root = DirectoryNode::new("/");

    let mut wz = DirectoryNode::new("/wz/");
    let file01 = Box::new(FileNode::new("/wz/a.txt"));
    println!("file files num: {}", file01.count_files());
    println!("file files num: {}", file01.count_file_size());
    wz.add(file01);

    let mut movies = DirectoryNode::new("/wz/movies/");
    // file 02
    movies.add(Box::new(FileNode::new("/wz/movies/c.avi")));
    wz.add(Box::new(movies));
    root.add(Box::new(wz));

    println!("directory files num: {}", root.count_files());
    println!("directory file_size num: {}", root.count_file_size());
}
