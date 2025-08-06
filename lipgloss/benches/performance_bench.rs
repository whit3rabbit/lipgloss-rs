//! Performance benchmarks for lipgloss operations.
//!
//! These benchmarks help verify that security improvements don't negatively
//! impact performance and that optimizations provide expected benefits.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lipgloss::{
    utils::{style_ranges, Range},
    Style,
};

fn benchmark_style_comparison(c: &mut Criterion) {
    let style1 = Style::new()
        .bold(true)
        .foreground("red")
        .background("blue")
        .width(50)
        .padding(2, 4, 2, 4);

    let style2 = Style::new()
        .bold(true)
        .foreground("red")
        .background("blue")
        .width(50)
        .padding(2, 4, 2, 4);

    c.bench_function("style_comparison_optimized", |b| {
        b.iter(|| black_box(style1.is_equivalent(black_box(&style2))))
    });
}

fn benchmark_render_performance(c: &mut Criterion) {
    let simple_style = Style::new().bold(true).foreground("red");
    let complex_style = Style::new()
        .bold(true)
        .italic(true)
        .foreground("red")
        .background("blue")
        .width(80)
        .padding(2, 4, 2, 4)
        .margin(1, 2, 1, 2);

    let content = "Hello, World!\nThis is a test.\nWith multiple lines.";

    c.bench_function("render_simple", |b| {
        b.iter(|| black_box(simple_style.render(black_box(content))))
    });

    c.bench_function("render_complex", |b| {
        b.iter(|| black_box(complex_style.render(black_box(content))))
    });

    // Note: render_optimized not yet implemented
    // c.bench_function("render_optimized_complex", |b| {
    //     b.iter(|| black_box(complex_style.render_optimized(black_box(content))))
    // });
}

fn benchmark_style_ranges(c: &mut Criterion) {
    let text = "The quick brown fox jumps over the lazy dog. ".repeat(20);
    let bold_style = Style::new().bold(true);
    let red_style = Style::new().foreground("red");
    let blue_style = Style::new().foreground("blue");

    let ranges = vec![
        Range::new(0, 10, bold_style.clone()),
        Range::new(20, 30, red_style.clone()),
        Range::new(40, 50, blue_style),
        Range::new(100, 120, bold_style),
        Range::new(200, 220, red_style),
    ];

    c.bench_function("style_ranges", |b| {
        b.iter(|| black_box(style_ranges(black_box(&text), black_box(&ranges))))
    });
}

fn benchmark_dimension_validation(c: &mut Criterion) {
    c.bench_function("dimension_validation", |b| {
        b.iter(|| {
            let _style = Style::new()
                .width(black_box(1000))
                .height(black_box(500))
                .padding(black_box(10), black_box(20), black_box(10), black_box(20));
        })
    });
}

fn benchmark_safe_repeat(c: &mut Criterion) {
    use lipgloss::security::{safe_repeat, safe_str_repeat};

    c.bench_function("safe_repeat_char", |b| {
        b.iter(|| black_box(safe_repeat(black_box(' '), black_box(100))))
    });

    c.bench_function("safe_repeat_str", |b| {
        b.iter(|| black_box(safe_str_repeat(black_box("abc"), black_box(50))))
    });
}

criterion_group!(
    benches,
    benchmark_style_comparison,
    benchmark_render_performance,
    benchmark_style_ranges,
    benchmark_dimension_validation,
    benchmark_safe_repeat
);
criterion_main!(benches);
