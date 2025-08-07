use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use lipgloss::{Color, Style};
use lipgloss_tree::{Leaf, Node, Tree};

fn build_nodes(path: &Path) -> io::Result<Vec<Box<dyn Node>>> {
    let mut entries: Vec<_> = fs::read_dir(path)?.filter_map(Result::ok).collect();
    // Sort for stable output
    entries.sort_by_key(|e| e.file_name());

    let mut nodes: Vec<Box<dyn Node>> = Vec::new();
    for entry in entries {
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        if name.starts_with('.') {
            // Skip hidden files and directories
            continue;
        }

        let entry_path = entry.path();
        if entry.file_type()?.is_dir() {
            // Directory: create a subtree and recurse
            let children = build_nodes(&entry_path)?;
            let subtree = Tree::new().root(name.to_string()).child(children);
            nodes.push(Box::new(subtree) as Box<dyn Node>);
        } else {
            // File: add a leaf
            nodes.push(Box::new(Leaf::new(name.to_string(), false)) as Box<dyn Node>);
        }
    }

    Ok(nodes)
}

fn main() -> io::Result<()> {
    let enumerator_style = Style::new().foreground(Color::from("240")).padding_right(1);
    let item_style = Style::new()
        .foreground(Color::from("99"))
        .bold(true)
        .padding_right(1);

    let pwd: PathBuf = env::current_dir()?;
    let pwd_str = pwd.to_string_lossy().to_string();

    let children = build_nodes(Path::new("."))?;
    let t = Tree::new()
        .root(&pwd_str)
        .enumerator_style(enumerator_style)
        .root_style(item_style.clone())
        .item_style(item_style)
        .child(children);

    println!("{}", t);
    Ok(())
}
