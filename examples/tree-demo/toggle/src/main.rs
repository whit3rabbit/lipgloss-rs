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
    let base = Style::new()
        .background(Color::from("57"))
        .foreground(Color::from("225"));
    let block = base
        .clone()
        .padding(1, 3, 1, 3)
        .margin(1, 3, 1, 3)
        .width(40);
    let enumerator = base.clone().foreground(Color::from("212")).padding_right(1);
    let styles = Styles {
        dir: base.clone(),
        toggle: base.clone().foreground(Color::from("207")).padding_right(1),
        file: base,
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
    println!("{}", block.render(&t.to_string()));
}
