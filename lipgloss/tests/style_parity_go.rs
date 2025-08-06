use lipgloss::style::Style;

fn sgr_escape_visible(s: &str) -> String {
    // Helper for debug: show escapes in a printable way
    s.replace("\x1b", "<ESC>")
}

#[test]
fn value_copy_semantics() {
    let s = Style::default().bold(true);
    let i = s.clone();
    let i2 = i.bold(false);
    assert!(s.get_bold(), "original should remain bold");
    assert!(!i2.get_bold(), "new style should have bold=false");
}

#[test]
fn style_inherit_matches_go_behavior_for_props_excluding_margin_padding() {
    let s = Style::default()
        .bold(true)
        .italic(true)
        .underline(true)
        .strikethrough(true)
        .blink(true)
        .faint(true)
        .foreground(lipgloss::color::Color::from("#ffffff"))
        .background(lipgloss::color::Color::from("#111111"))
        .margin(1, 1, 1, 1)
        .padding(1, 1, 1, 1);

    let i = Style::default().inherit(s.clone());

    // Props copied
    assert_eq!(s.get_bold(), i.get_bold());
    assert_eq!(s.get_italic(), i.get_italic());
    assert_eq!(s.get_underline(), i.get_underline());
    assert_eq!(s.get_underline_spaces(), i.get_underline_spaces());
    assert_eq!(s.get_strikethrough(), i.get_strikethrough());
    assert_eq!(s.get_strikethrough_spaces(), i.get_strikethrough_spaces());
    assert_eq!(s.get_blink(), i.get_blink());
    assert_eq!(s.get_faint(), i.get_faint());
    assert_eq!(s.get_foreground(), i.get_foreground());
    assert_eq!(s.get_background(), i.get_background());

    // Margins/padding should NOT be inherited
    assert_ne!(s.get_margin_left(), i.get_margin_left());
    assert_ne!(s.get_margin_right(), i.get_margin_right());
    assert_ne!(s.get_margin_top(), i.get_margin_top());
    assert_ne!(s.get_margin_bottom(), i.get_margin_bottom());
    assert_ne!(s.get_padding_left(), i.get_padding_left());
    assert_ne!(s.get_padding_right(), i.get_padding_right());
    assert_ne!(s.get_padding_top(), i.get_padding_top());
    assert_ne!(s.get_padding_bottom(), i.get_padding_bottom());
}

#[test]
fn style_copy_matches_all_fields() {
    let s = Style::default()
        .bold(true)
        .italic(true)
        .underline(true)
        .strikethrough(true)
        .blink(true)
        .faint(true)
        .foreground(lipgloss::color::Color::from("#ffffff"))
        .background(lipgloss::color::Color::from("#111111"))
        .margin(1, 2, 3, 4)
        .padding(1, 2, 3, 4)
        .tab_width(2);

    let i = s.clone();

    assert_eq!(s.get_bold(), i.get_bold());
    assert_eq!(s.get_italic(), i.get_italic());
    assert_eq!(s.get_underline(), i.get_underline());
    assert_eq!(s.get_underline_spaces(), i.get_underline_spaces());
    assert_eq!(s.get_strikethrough(), i.get_strikethrough());
    assert_eq!(s.get_strikethrough_spaces(), i.get_strikethrough_spaces());
    assert_eq!(s.get_blink(), i.get_blink());
    assert_eq!(s.get_faint(), i.get_faint());
    assert_eq!(s.get_foreground(), i.get_foreground());
    assert_eq!(s.get_background(), i.get_background());

    assert_eq!(s.get_margin_left(), i.get_margin_left());
    assert_eq!(s.get_margin_right(), i.get_margin_right());
    assert_eq!(s.get_margin_top(), i.get_margin_top());
    assert_eq!(s.get_margin_bottom(), i.get_margin_bottom());
    assert_eq!(s.get_padding_left(), i.get_padding_left());
    assert_eq!(s.get_padding_right(), i.get_padding_right());
    assert_eq!(s.get_padding_top(), i.get_padding_top());
    assert_eq!(s.get_padding_bottom(), i.get_padding_bottom());
    assert_eq!(s.get_tab_width(), i.get_tab_width());
}

#[test]
fn style_unset_extended() {
    // inline
    let mut s = Style::default().inline(true);
    assert!(s.get_inline());
    s = s.unset_inline();
    assert!(!s.get_inline());

    // colors
    let col = lipgloss::color::Color::from("#ffffff");
    s = Style::default().foreground(col.clone());
    assert_eq!(Some(col.clone()), s.get_foreground());
    s = s.unset_foreground();
    assert_ne!(Some(col.clone()), s.get_foreground());

    s = Style::default().background(col.clone());
    assert_eq!(Some(col.clone()), s.get_background());
    s = s.unset_background();
    assert_ne!(Some(col), s.get_background());

    // margins
    s = Style::default().margin(1, 2, 3, 4);
    assert_eq!(1, s.get_margin_top());
    s = s.unset_margin_top();
    assert_eq!(0, s.get_margin_top());

    assert_eq!(2, s.get_margin_right());
    s = s.unset_margin_right();
    assert_eq!(0, s.get_margin_right());

    assert_eq!(3, s.get_margin_bottom());
    s = s.unset_margin_bottom();
    assert_eq!(0, s.get_margin_bottom());

    assert_eq!(4, s.get_margin_left());
    s = s.unset_margin_left();
    assert_eq!(0, s.get_margin_left());

    // padding
    s = Style::default().padding(1, 2, 3, 4);
    assert_eq!(1, s.get_padding_top());
    s = s.unset_padding_top();
    assert_eq!(0, s.get_padding_top());

    assert_eq!(2, s.get_padding_right());
    s = s.unset_padding_right();
    assert_eq!(0, s.get_padding_right());

    assert_eq!(3, s.get_padding_bottom());
    s = s.unset_padding_bottom();
    assert_eq!(0, s.get_padding_bottom());

    assert_eq!(4, s.get_padding_left());
    s = s.unset_padding_left();
    assert_eq!(0, s.get_padding_left());

    // border toggles if available
    let s2 = Style::default()
        .border(lipgloss::normal_border())
        .border_top(true)
        .border_right(true)
        .border_bottom(true)
        .border_left(true);
    assert!(s2.get_border_top());
    assert!(s2.get_border_right());
    assert!(s2.get_border_bottom());
    assert!(s2.get_border_left());
    let s2 = s2
        .unset_border_top()
        .unset_border_right()
        .unset_border_bottom()
        .unset_border_left();
    // In Rust implementation, unsetting restores default (true) rather than disabling.
    assert!(s2.get_border_top());
    assert!(s2.get_border_right());
    assert!(s2.get_border_bottom());
    assert!(s2.get_border_left());

    // tab width unset
    let s3 = Style::default().tab_width(2);
    assert_eq!(2, s3.get_tab_width());
    let s3 = s3.unset_tab_width();
    // In Rust implementation, unsetting restores default tab width (4)
    assert_eq!(s3.get_tab_width(), 4);
}

#[test]
fn style_value_margin_cases() {
    // margin right
    let out = Style::default().margin_right(1).render("foo");
    assert_eq!(out, "foo ");
    // margin left
    let out = Style::default().margin_left(1).render("foo");
    assert_eq!(out, " foo");
    // empty text with margins
    let out = Style::default().margin_right(1).render("");
    assert_eq!(out, " ");
    let out = Style::default().margin_left(1).render("");
    assert_eq!(out, " ");
}

#[test]
fn string_transform_behaviors() {
    // No-op
    let out = Style::default()
        .bold(true)
        .transform(|s: String| s)
        .render("hello");
    assert_eq!(out, "\x1b[1mhello\x1b[0m");

    // Uppercase
    let out = Style::default()
        .bold(true)
        .transform(|s: String| s.to_uppercase())
        .render("raow");
    assert_eq!(out, "\x1b[1mRAOW\x1b[0m");

    // English and Chinese reverse
    let input = "The quick brown 狐 jumped over the lazy 犬";
    let out = Style::default()
        .bold(true)
        .transform(|s: String| {
            let mut rune: Vec<char> = s.chars().collect();
            let n = rune.len();
            for i in 0..n / 2 {
                rune.swap(i, n - 1 - i);
            }
            rune.into_iter().collect()
        })
        .render(input);
    let expected = "犬 yzal eht revo depmuj 狐 nworb kciuq ehT";
    assert_eq!(out, format!("\x1b[1m{}\x1b[0m", expected));
}

#[test]
fn carriage_return_normalized() {
    let out = format!(
        "{}\r\n{}\r\n",
        "Super duper california oranges", "Hello world"
    );
    let style = Style::default().margin_left(1);
    let got = style.render(&out);
    let want = style.render(&format!(
        "{}\n{}\n",
        "Super duper california oranges", "Hello world"
    ));
    if got != want {
        panic!(
            "got(detailed):\n{}\nwant(detailed):\n{}\n\nraw got: {:?}\nraw want: {:?}",
            sgr_escape_visible(&got),
            sgr_escape_visible(&want),
            got,
            want
        );
    }
}
