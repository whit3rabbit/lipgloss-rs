use lipgloss::{Color, Style};
use lipgloss_tree::{rounded_enumerator, Leaf, Node, Tree};

fn main() {
    println!("--- Simple Tree ---");
    simple_tree();

    println!("\n--- Styled Tree ---");
    styled_tree();

    println!("\n--- Background Tree ---");
    background_tree();

    println!("\n--- Makeup Tree ---");
    makeup_tree();

    println!("\n--- Toggle Tree ---");
    toggle_tree();
}

fn simple_tree() {
    let t = Tree::new().root(".").child(vec![
        Box::new(Leaf::new("macOS", false)) as Box<dyn Node>,
        Box::new(Tree::new().root("Linux").child(vec![
            Box::new(Leaf::new("NixOS", false)) as Box<dyn Node>,
            Box::new(Leaf::new("Arch Linux (btw)", false)) as Box<dyn Node>,
            Box::new(Leaf::new("Void Linux", false)) as Box<dyn Node>,
        ])) as Box<dyn Node>,
        Box::new(Tree::new().root("BSD").child(vec![
            Box::new(Leaf::new("FreeBSD", false)) as Box<dyn Node>,
            Box::new(Leaf::new("OpenBSD", false)) as Box<dyn Node>,
        ])) as Box<dyn Node>,
    ]);

    println!("{}", t);
}

fn styled_tree() {
    let purple = Style::new().foreground(Color::from("99")).margin_right(1);
    let pink = Style::new().foreground(Color::from("212")).margin_right(1);

    let t = Tree::new()
        .child(vec![
            Box::new(Leaf::new("Glossier", false)) as Box<dyn Node>,
            Box::new(Leaf::new("Claire's Boutique", false)) as Box<dyn Node>,
            Box::new(
                Tree::new()
                    .root("Nyx")
                    .child(vec![
                        Box::new(Leaf::new("Lip Gloss", false)) as Box<dyn Node>,
                        Box::new(Leaf::new("Foundation", false)) as Box<dyn Node>,
                    ])
                    .enumerator_style(pink),
            ) as Box<dyn Node>,
            Box::new(Leaf::new("Mac", false)) as Box<dyn Node>,
            Box::new(Leaf::new("Milk", false)) as Box<dyn Node>,
        ])
        .enumerator_style(purple);

    println!("{}", t);
}

fn background_tree() {
    let enumerator_style = Style::new()
        .background(Color::from("0"))
        .padding(0, 1, 0, 1);
    let header_item_style = Style::new()
        .background(Color::from("#ee6ff8"))
        .foreground(Color::from("#ecfe65"))
        .bold(true)
        .padding(0, 1, 0, 1);
    let item_style = header_item_style.clone().background(Color::from("0"));

    let t = Tree::new()
        .root("# Table of Contents")
        .root_style(item_style.clone())
        .item_style(item_style)
        .enumerator_style(enumerator_style)
        .child(vec![
            Box::new(Tree::new().root("## Chapter 1").child(vec![
                Box::new(Leaf::new("Chapter 1.1", false)) as Box<dyn Node>,
                Box::new(Leaf::new("Chapter 1.2", false)) as Box<dyn Node>,
            ])) as Box<dyn Node>,
            Box::new(Tree::new().root("## Chapter 2").child(vec![
                Box::new(Leaf::new("Chapter 2.1", false)) as Box<dyn Node>,
                Box::new(Leaf::new("Chapter 2.2", false)) as Box<dyn Node>,
            ])) as Box<dyn Node>,
        ]);

    println!("{}", t);
}

fn makeup_tree() {
    let enumerator_style = Style::new().foreground(Color::from("63")).margin_right(1);
    let root_style = Style::new().foreground(Color::from("35"));
    let item_style = Style::new().foreground(Color::from("212"));

    let t = Tree::new()
        .root("⁜ Makeup")
        .child(vec![
            Box::new(Leaf::new("Glossier", false)) as Box<dyn Node>,
            Box::new(Leaf::new("Fenty Beauty", false)) as Box<dyn Node>,
            Box::new(Tree::new().child(vec![
                Box::new(Leaf::new("Gloss Bomb Universal Lip Luminizer", false)) as Box<dyn Node>,
                Box::new(Leaf::new("Hot Cheeks Velour Blushlighter", false)) as Box<dyn Node>,
            ])) as Box<dyn Node>,
            Box::new(Leaf::new("Nyx", false)) as Box<dyn Node>,
            Box::new(Leaf::new("Mac", false)) as Box<dyn Node>,
            Box::new(Leaf::new("Milk", false)) as Box<dyn Node>,
        ])
        .enumerator(rounded_enumerator)
        .enumerator_style(enumerator_style)
        .root_style(root_style)
        .item_style(item_style);
    println!("{}", t);
}

// --- Toggle Tree (Advanced) ---

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

fn toggle_tree() {
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
