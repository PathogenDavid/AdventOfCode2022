fn main() {
    //=============================================================================================
    // Build a file system tree from the terminal output
    //=============================================================================================
    let terminal_output = include_str!("day7.txt").trim().replace("\r", "");

    let mut root = FsNode::dir("/".into());
    let mut terminal_output = terminal_output.lines();

    assert_eq!(Some("$ cd /"), terminal_output.next());
    process_output(&mut root, &mut terminal_output);

    // Calculate the size of the whole tree
    root.calc_size();

    fn process_output<'a, I>(root: &mut FsNode, terminal_output: &mut I)
    where
        I: Iterator<Item = &'a str>,
    {
        while let Some(line) = terminal_output.next() {
            const CD_PREFIX: &str = "$ cd ";
            const DIR_PREFIX: &str = "dir ";

            if line == "$ ls" {
                continue;
            } else if line.starts_with(CD_PREFIX) {
                let name = &line[CD_PREFIX.len()..];
                if name == ".." {
                    return;
                } else if name == "/" {
                    todo!();
                } else {
                    let cd = &mut root.find_dir(name).expect("cd went to non-existent directory");
                    process_output(cd, terminal_output);
                }
            } else if line.starts_with(DIR_PREFIX) {
                let name = &line[DIR_PREFIX.len()..];
                root.add_child(FsNode::dir(name));
            } else {
                let (size, name) = line.split_once(' ').unwrap();
                let size: usize = size.parse().unwrap();
                root.add_child(FsNode::file(name, size));
            }
        }
    }

    //=============================================================================================
    // Part 1 - Get the small directory total
    //=============================================================================================
    println!("Part 1: {}", small_directory_total(&root));
    fn small_directory_total(node: &FsNode) -> usize {
        match node {
            FsNode::File { .. } => 0,
            FsNode::Dir { children, size, .. } => {
                let mut size = *size;

                // Don't count this directory if it's too big
                if size >= 100_000 {
                    size = 0;
                }

                size + children
                    .iter()
                    .map(|c| small_directory_total(c))
                    .sum::<usize>()
            }
        }
    }

    //=============================================================================================
    // Part 2 - Determine which directories to delete
    //=============================================================================================
    const TOTAL_FS_SIZE: usize = 70_000_000;
    const SPACED_NEEDED: usize = 30_000_000;

    let free_bytes = TOTAL_FS_SIZE - root.size();
    let target_size = SPACED_NEEDED - free_bytes;

    // Enumerate all directories on the filesystem and sort them from smallest to largest
    let mut all_directories: Vec<&FsNode> = vec![];
    enumerate_directories(&mut all_directories, &root);

    all_directories.sort_by_key(|d| d.size());

    fn enumerate_directories<'a>(all_directories: &mut Vec<&'a FsNode>, node: &'a FsNode) {
        if let FsNode::Dir { children, .. } = node {
            all_directories.push(node);
            for child in children {
                enumerate_directories(all_directories, child);
            }
        }
    }

    // Find a directory that is at least target_size bytes
    let mut found_size = 0;
    for directory in all_directories {
        if directory.size() > target_size {
            found_size = directory.size();
            break;
        }
    }

    println!("Part 2: {found_size}");
}

enum FsNode {
    File {
        name: String,
        size: usize,
    },
    Dir {
        name: String,
        children: Vec<FsNode>,
        size: usize,
    },
}

impl FsNode {
    fn dir(name: &str) -> FsNode {
        FsNode::Dir {
            name: name.into(),
            children: vec![],
            size: 0,
        }
    }

    fn file(name: &str, size: usize) -> FsNode {
        FsNode::File {
            name: name.into(),
            size,
        }
    }

    fn calc_size(&mut self) -> usize {
        match self {
            FsNode::File { size, .. } => *size,
            FsNode::Dir { size, children, .. } => {
                *size = children.iter_mut().map(|c| c.calc_size()).sum();
                *size
            }
        }
    }

    // This is a britle API since it relies on calc_size having been called
    fn size(&self) -> usize {
        match self {
            FsNode::File { size, .. } => *size,
            FsNode::Dir { size, .. } => *size,
        }
    }

    fn add_child(&mut self, child: FsNode) {
        match self {
            FsNode::File { .. } => panic!("cannot add children to a file node"),
            FsNode::Dir { children, .. } => children.push(child),
        }
    }

    fn find_dir(&mut self, search_name: &str) -> Option<&mut FsNode> {
        match self {
            FsNode::File { .. } => None,
            FsNode::Dir { children, .. } => {
                for child in children {
                    let child_name = match child {
                        FsNode::File { name, .. } => name,
                        FsNode::Dir { name, .. } => name,
                    };

                    if child_name == search_name {
                        return Some(child);
                    }
                }

                None
            }
        }
    }
}
