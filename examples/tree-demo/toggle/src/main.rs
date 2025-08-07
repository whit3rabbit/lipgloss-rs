use lipgloss::{Color, Style};
use lipgloss_tree::{rounded_enumerator, Leaf, Node, Tree};

#[derive(Clone)]
struct Dir {
    name: String,
    open: bool,
    styles: Styles,
}

#[derive(Clone)]
struct File {
    name: String,
    styles: Styles,
}

#[derive(Clone)]
struct Styles {
    dir: Style,
    toggle: Style,
    file: Style,
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let toggle = self.styles.toggle.render(if self.open { "▼" } else { "▶" });
        let name = self.styles.dir.render(&self.name);
        write!(f, "{}{}", toggle, name)
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.styles.file.render(&self.name))
    }
}

fn default_styles() -> (Styles, Style, Style) {
    // Base style with only foreground color for tree components to inherit.
    // Background color is applied only by the outer block to prevent ANSI reset conflicts.
    let base = Style::new().foreground(Color::from("225"));

    // The block style for the outer box with explicit background and color_whitespace.
    // This ensures ALL whitespace within the block width gets painted with the background.
    let block = Style::new()
        .background(Color::from("57"))
        .padding(1, 3, 1, 3)
        .margin(1, 3, 1, 3)
        .width(40)
        .color_whitespace(true);

    // The style for tree branches, inheriting the base background.
    let enumerator = base.clone().foreground(Color::from("212")).padding_right(1);

    // Styles for tree content (directories, toggles, files), all inheriting the base background.
    let styles = Styles {
        dir: base.clone().inline(true),
        toggle: base.clone().foreground(Color::from("207")).padding_right(1),
        file: base.clone(),
    };

    (styles, block, enumerator)
}

fn main() {
    let (s, block, enumerator_style) = default_styles();

    let t = Tree::new()
        .root(
            Dir {
                name: "~/charm".to_string(),
                open: true,
                styles: s.clone(),
            }
            .to_string(),
        )
        .enumerator(rounded_enumerator)
        .enumerator_style(enumerator_style)
        .child(vec![
            Box::new(Leaf::new(
                Dir {
                    name: "ayman".to_string(),
                    open: false,
                    styles: s.clone(),
                }
                .to_string(),
                false,
            )) as Box<dyn Node>,
            Box::new(
                Tree::new()
                    .root(
                        Dir {
                            name: "bash".to_string(),
                            open: true,
                            styles: s.clone(),
                        }
                        .to_string(),
                    )
                    .child(vec![Box::new(
                        Tree::new()
                            .root(
                                Dir {
                                    name: "tools".to_string(),
                                    open: true,
                                    styles: s.clone(),
                                }
                                .to_string(),
                            )
                            .child(vec![
                                Box::new(Leaf::new(
                                    File {
                                        name: "zsh".to_string(),
                                        styles: s.clone(),
                                    }
                                    .to_string(),
                                    false,
                                )) as Box<dyn Node>,
                                Box::new(Leaf::new(
                                    File {
                                        name: "doom-emacs".to_string(),
                                        styles: s.clone(),
                                    }
                                    .to_string(),
                                    false,
                                )) as Box<dyn Node>,
                            ]),
                    ) as Box<dyn Node>]),
            ) as Box<dyn Node>,
            Box::new(
                Tree::new()
                    .root(
                        Dir {
                            name: "carlos".to_string(),
                            open: true,
                            styles: s.clone(),
                        }
                        .to_string(),
                    )
                    .child(vec![Box::new(
                        Tree::new()
                            .root(
                                Dir {
                                    name: "emotes".to_string(),
                                    open: true,
                                    styles: s.clone(),
                                }
                                .to_string(),
                            )
                            .child(vec![
                                Box::new(Leaf::new(
                                    File {
                                        name: "chefkiss.png".to_string(),
                                        styles: s.clone(),
                                    }
                                    .to_string(),
                                    false,
                                )) as Box<dyn Node>,
                                Box::new(Leaf::new(
                                    File {
                                        name: "kekw.png".to_string(),
                                        styles: s.clone(),
                                    }
                                    .to_string(),
                                    false,
                                )) as Box<dyn Node>,
                            ]),
                    ) as Box<dyn Node>]),
            ) as Box<dyn Node>,
            Box::new(Leaf::new(
                Dir {
                    name: "maas".to_string(),
                    open: false,
                    styles: s,
                }
                .to_string(),
                false,
            )) as Box<dyn Node>,
        ]);
    // Render the tree and apply block styling with background
    // Remove reset codes from tree content to prevent background interference
    let tree_content = t.to_string();
    let cleaned_content = tree_content.replace("\x1b[0m", "");
    println!("{}", block.render(&cleaned_content));
}
