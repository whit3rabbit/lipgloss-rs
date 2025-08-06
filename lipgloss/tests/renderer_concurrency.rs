use std::sync::{Arc, Barrier};
use std::thread;

use lipgloss::renderer::*;

#[test]
fn concurrent_reads_and_writes_on_renderer() {
    let mut base = Renderer::new();
    base.set_color_profile(ColorProfileKind::ANSI256);
    base.set_has_dark_background(true);

    let shared = Arc::new(base);
    let threads = 8;
    let barrier = Arc::new(Barrier::new(threads));
    let mut handles = Vec::with_capacity(threads);

    for _ in 0..threads {
        let r = shared.clone();
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            // Start together
            b.wait();
            // Concurrent read-only access on the same Renderer instance
            for _ in 0..20_000 {
                let _ = r.color_profile();
                let _ = r.has_dark_background();
            }
        }));
    }

    for h in handles {
        h.join().expect("thread panicked");
    }
}

#[test]
fn concurrent_global_default_mutation_and_reads() {
    // Ensure default exists
    let _ = default_renderer();

    let threads = 6;
    let barrier = Arc::new(Barrier::new(threads));
    let mut handles = Vec::with_capacity(threads);

    for i in 0..threads {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            b.wait();
            for j in 0..20_000 {
                if (i + j) % 11 == 0 {
                    // Replace default renderer occasionally
                    let mut r = Renderer::new();
                    if j % 2 == 0 {
                        r.set_color_profile(ColorProfileKind::TrueColor);
                    } else {
                        r.set_color_profile(ColorProfileKind::NoColor);
                    }
                    r.set_has_dark_background((i + j) % 5 != 0);
                    set_default_renderer(r);
                } else if (i + j) % 5 == 0 {
                    // Global setter paths
                    set_has_dark_background(((i + j) % 3) == 0);
                    set_color_profile(if (i + j) % 2 == 0 {
                        ColorProfileKind::ANSI
                    } else {
                        ColorProfileKind::ANSI256
                    });
                } else {
                    // Reads from default
                    let _ = color_profile();
                    let _ = has_dark_background();
                }
            }
        }));
    }

    for h in handles {
        h.join().expect("thread panicked");
    }
}
