use lipgloss::{NewRange, Range, Style, StyleRanges, StyleRunes};

fn strip_ansi(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            // skip SGR until 'm'
            for nc in chars.by_ref() {
                if nc == 'm' {
                    break;
                }
            }
            continue;
        }
        out.push(c);
    }
    out
}

#[test]
fn test_style_ranges_basic() {
    let base = "Hello World";
    let bold = Style::new().bold(true);
    let r = Range::new(0, 5, bold);
    let out = StyleRanges(base, &[r]);

    // Text content preserved
    assert_eq!(strip_ansi(&out), base);

    // Expect the first 5 chars to be wrapped with SGR codes (bold = 1)
    assert!(out.contains("\x1b[1mHello\x1b[0m"));
    // And the remainder should be unstyled plain text
    assert!(out.ends_with(" World"));
}

#[test]
fn test_style_ranges_overlap_order() {
    let base = "abcdef";
    let bold = Style::new().bold(true);
    let underline = Style::new().underline(true);

    // Overlapping ranges applied in input order
    let r1 = NewRange(0, 4, bold.clone()); // styles abcd
    let r2 = NewRange(2, 6, underline.clone()); // styles cdef
    let out = StyleRanges(base, &[r1, r2]);

    // a b should be bold
    assert!(out.contains("\x1b[1mab\x1b[0m"));
    // c d should be underlined (grouping may coalesce to a larger segment)
    assert!(
        out.contains("\x1b[4mc\x1b[0m")
            || out.contains("\x1b[4mcd\x1b[0m")
            || out.contains("\x1b[4mcdef\x1b[0m")
    );
    // e f should be underlined (may be part of a larger grouped segment like cdef)
    assert!(
        out.contains("\x1b[4mef\x1b[0m")
            || (out.contains("\x1b[4me\x1b[0m") && out.contains("\x1b[4mf\x1b[0m"))
            || out.contains("\x1b[4mcdef\x1b[0m")
    );

    // Content intact
    assert_eq!(strip_ansi(&out), base);
}

#[test]
fn test_style_ranges_with_cjk() {
    let base = "中ABCD"; // first char is width-2 CJK
    let italic = Style::new().italic(true);
    let out = StyleRanges(base, &[Range::new(0, 1, italic)]);
    assert!(out.starts_with("\x1b[3m中\x1b[0m"));
    assert_eq!(strip_ansi(&out), base);
}

#[test]
fn test_style_runes_indices() {
    let base = "012345";
    let matched = Style::new().bold(true);
    let unmatched = Style::new().faint(true);
    let out = StyleRunes(base, &[0, 1, 2, 2, 1], matched, unmatched);

    // First three characters should appear in bold segments
    assert!(out.contains("\x1b[1m0\x1b[0m"));
    assert!(out.contains("\x1b[1m1\x1b[0m"));
    assert!(out.contains("\x1b[1m2\x1b[0m"));
    // Others should have faint styling somewhere
    assert!(out.contains("\x1b[2m3") || out.contains("\x1b[2m34") || out.contains("\x1b[2m345"));
    assert_eq!(strip_ansi(&out), base);
}
