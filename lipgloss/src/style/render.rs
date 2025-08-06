//! Text rendering with ANSI escape sequence generation.
//!
//! This module provides the core rendering functionality that converts styled text
//! into terminal-ready output with appropriate ANSI escape sequences for colors,
//! attributes, borders, spacing, and layout.

use crate::color::parse_hex_rgba;
use crate::renderer::{default_renderer, ColorProfileKind};
use crate::security::{safe_repeat, safe_str_repeat};
use crate::style::{properties::*, Style};
use crate::width_visible;

impl Style {
    /// Renders text with all configured style properties applied.
    ///
    /// This method applies the complete style configuration to the provided text,
    /// generating ANSI escape sequences for terminal display. It handles:
    ///
    /// - Text attributes (bold, italic, underline, etc.)
    /// - Foreground and background colors
    /// - Borders and border colors
    /// - Padding and margins
    /// - Text alignment and positioning
    /// - Size constraints and content wrapping
    ///
    /// # Arguments
    ///
    /// * `s` - The text content to render with this style
    ///
    /// # Returns
    ///
    /// A string containing the input text wrapped with appropriate ANSI escape
    /// sequences and formatting to display the styled content in the terminal.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// // Create a style and render text with it
    /// let style = Style::default();
    /// let output = style.render("Hello, World!");
    /// // Returns "Hello, World!" with any configured style properties applied
    /// ```
    ///
    /// # Performance
    ///
    /// This method efficiently builds ANSI sequences by only including codes for
    /// properties that have been explicitly set on the style, minimizing output size.
    pub fn render(&self, s: &str) -> String {
        // Content to render: prefer internal value if set
        let content = if !self.value.is_empty() {
            self.value.clone()
        } else {
            s.to_string()
        };

        // First, build the base rendered string before colorizing: borders, padding, etc.
        let mut rendered = content.clone();

        // Normalize newlines: convert CRLF/CR to LF
        if rendered.contains('\r') {
            rendered = rendered.replace("\r\n", "\n");
            rendered = rendered.replace('\r', "\n");
        }

        // Inline: remove newlines if inline=true
        if self.get_attr(ATTR_INLINE) && self.is_set(INLINE_KEY) {
            rendered = rendered.replace('\n', "");
        }

        // Apply transform, if any
        if self.is_set(TRANSFORM_KEY) {
            if let Some(ref f) = self.transform {
                rendered = f(rendered);
            }
        }

        // Tabs handling: default 4 spaces, 0 removes, -1 keeps as-is, n>0 replaces with n spaces
        let tabw = self.get_tab_width();
        if tabw == 0 {
            rendered = rendered.replace('\t', "");
        } else if tabw > 0 {
            let spaces = safe_repeat(' ', tabw as usize);
            rendered = rendered.replace('\t', &spaces);
        } // tabw < 0 => keep tabs as-is

        // Max height truncation
        let mh = self.get_max_height();
        if mh > 0 {
            let lines: Vec<&str> = rendered.split('\n').collect();
            if (lines.len() as i32) > mh {
                rendered = lines[..mh as usize].join("\n");
            }
        }

        // Max width truncation per line (ANSI-aware)
        let mw = self.get_max_width();
        if mw > 0 {
            let lines: Vec<&str> = rendered.split('\n').collect();
            let mut out_lines: Vec<String> = Vec::with_capacity(lines.len());
            for line in lines {
                out_lines.push(Self::truncate_visible_line(line, mw as usize));
            }
            rendered = out_lines.join("\n");
        }

        // Word wrap when width > 0
        // The width should be the total width INCLUDING padding, so we need to
        // subtract padding from the width before word wrapping
        let w_setting = self.get_width();
        if w_setting > 0 {
            let pad_l = if self.is_set(PADDING_LEFT_KEY) {
                self.get_padding_left().max(0) as i32
            } else {
                0
            };
            let pad_r = if self.is_set(PADDING_RIGHT_KEY) {
                self.get_padding_right().max(0) as i32
            } else {
                0
            };
            
            // Calculate the actual content width by subtracting horizontal padding
            let content_width = (w_setting - pad_l - pad_r).max(0);
            
            if content_width > 0 {
                let mut wrapped_lines: Vec<String> = Vec::new();
                for line in rendered.split('\n') {
                    // FIX: Call the new word wrap function instead of hard wrap.
                    let mut parts = Self::word_wrap_ansi_aware(line, content_width as usize);
                    if parts.is_empty() {
                        parts.push(String::new());
                    }
                    wrapped_lines.extend(parts.into_iter());
                }
                rendered = wrapped_lines.join("\n");
            }
        }

        // Horizontal alignment is now handled in the final intelligent styling pass

        // Padding left/right applied per line
        let pad_l = if self.is_set(PADDING_LEFT_KEY) {
            self.get_padding_left().max(0) as usize
        } else {
            0
        };
        let pad_r = if self.is_set(PADDING_RIGHT_KEY) {
            self.get_padding_right().max(0) as usize
        } else {
            0
        };
        if pad_l > 0 || pad_r > 0 {
            let lines: Vec<&str> = rendered.split('\n').collect();
            let mut padded: Vec<String> = Vec::with_capacity(lines.len());
            let lp = safe_repeat(' ', pad_l);
            let rp = safe_repeat(' ', pad_r);
            for line in lines {
                padded.push(format!("{}{}{}", lp, line, rp));
            }
            rendered = padded.join("\n");
        }

        // Vertical padding (top/bottom) - add empty lines
        let pad_t = if self.is_set(PADDING_TOP_KEY) {
            self.get_padding_top().max(0) as usize
        } else {
            0
        };
        let pad_b = if self.is_set(PADDING_BOTTOM_KEY) {
            self.get_padding_bottom().max(0) as usize
        } else {
            0
        };
        if pad_t > 0 || pad_b > 0 {
            let mut lines = Vec::new();
            
            // Add top padding lines
            if pad_t > 0 {
                lines.extend(vec![String::new(); pad_t]);
            }
            
            // Add existing content
            lines.extend(rendered.split('\n').map(|s| s.to_string()));
            
            // Add bottom padding lines
            if pad_b > 0 {
                lines.extend(vec![String::new(); pad_b]);
            }
            
            rendered = lines.join("\n");
        }

        // NOTE: Borders and margins are now handled after layout constraints to match Go implementation

        // Build ANSI SGR sequence from current style settings for text content.
        let mut sgr: Vec<String> = Vec::new();
        let eff = self.r.clone().unwrap_or_else(|| default_renderer().clone());
        let profile = eff.color_profile();

        // Text attributes
        if self.get_attr(ATTR_BOLD) && self.is_set(BOLD_KEY)
        {
            sgr.push("1".to_string());
        }
        if self.get_attr(ATTR_FAINT) && self.is_set(FAINT_KEY)
        {
            sgr.push("2".to_string());
        }
        if self.get_attr(ATTR_ITALIC) && self.is_set(ITALIC_KEY)
        {
            sgr.push("3".to_string());
        }
        if self.get_attr(ATTR_UNDERLINE) && self.is_set(UNDERLINE_KEY)
        {
            sgr.push("4".to_string());
        }
        if self.get_attr(ATTR_BLINK) && self.is_set(BLINK_KEY)
        {
            sgr.push("5".to_string());
        }
        if self.get_attr(ATTR_REVERSE) && self.is_set(REVERSE_KEY)
        {
            sgr.push("7".to_string());
        }
        if self.get_attr(ATTR_STRIKETHROUGH) && self.is_set(STRIKETHROUGH_KEY)
        {
            sgr.push("9".to_string());
        }

        // Foreground color
        if !matches!(profile, ColorProfileKind::NoColor)
            && self.is_set(FOREGROUND_KEY)
        {
            if let Some(ref tok) = self.fg_color {
                if tok.starts_with('#') {
                    if let Some((r, g, b, _a)) = parse_hex_rgba(tok) {
                        match profile {
                            ColorProfileKind::TrueColor => {
                                // RGB values are already 8-bit (0-255) cast to u32
                                sgr.push(format!("38;2;{};{};{}", r, g, b));
                            }
                            ColorProfileKind::ANSI | ColorProfileKind::ANSI256 => {
                                sgr.push("38;5;0".to_string())
                            }
                            ColorProfileKind::NoColor => {}
                        }
                    }
                } else if let Ok(idx) = tok.parse::<u32>() {
                    let idx = idx % 256;
                    match profile {
                        ColorProfileKind::TrueColor => {
                            // best-effort: still use indexed if we don't have original hex
                            sgr.push(format!("38;5;{}", idx));
                        }
                        ColorProfileKind::ANSI => {
                            // For ANSI profile, handle direct ANSI codes and mapped colors
                            if idx <= 7 {
                                sgr.push(format!("{}", 30 + idx)); // 30-37 (standard colors)
                            } else if idx <= 15 {
                                sgr.push(format!("{}", 82 + idx)); // 90-97 (82+8=90, 82+15=97, bright colors)
                            } else if idx >= 30 && idx <= 37 {
                                sgr.push(format!("{}", idx)); // Direct ANSI codes 30-37
                            } else if idx >= 90 && idx <= 97 {
                                sgr.push(format!("{}", idx)); // Direct ANSI codes 90-97
                            } else {
                                sgr.push("39".to_string()); // default foreground
                            }
                        }
                        ColorProfileKind::ANSI256 => {
                            sgr.push(format!("38;5;{}", idx))
                        }
                        ColorProfileKind::NoColor => {}
                    }
                }
            }
        }

        // Background color
        if !matches!(profile, ColorProfileKind::NoColor)
            && self.is_set(BACKGROUND_KEY)
        {
            if let Some(ref tok) = self.bg_color {
                if tok.starts_with('#') {
                    if let Some((r, g, b, _a)) = parse_hex_rgba(tok) {
                        match profile {
                            ColorProfileKind::TrueColor => {
                                // RGB values are already 8-bit (0-255) cast to u32
                                sgr.push(format!("48;2;{};{};{}", r, g, b));
                            }
                            ColorProfileKind::ANSI256 => {
                                // Convert RGB to ANSI256 background color
                                let ansi256_idx = crate::color::rgb_to_ansi256(r as u8, g as u8, b as u8);
                                sgr.push(format!("48;5;{}", ansi256_idx));
                            }
                            ColorProfileKind::ANSI => {
                                // Convert RGB to ANSI16 background color
                                let ansi16_idx = crate::color::rgb_to_ansi16(r as u8, g as u8, b as u8);
                                sgr.push(format!("{}", 40 + ansi16_idx));
                            }
                            ColorProfileKind::NoColor => {}
                        }
                    }
                } else if let Ok(idx) = tok.parse::<u32>() {
                    let idx = idx % 256;
                    match profile {
                        ColorProfileKind::TrueColor => sgr.push(format!("48;5;{}", idx)),
                        ColorProfileKind::ANSI => {
                            // For ANSI profile, handle direct ANSI codes and mapped colors
                            if idx <= 7 {
                                sgr.push(format!("{}", 40 + idx)); // 40-47 (standard colors)
                            } else if idx <= 15 {
                                sgr.push(format!("{}", 92 + idx)); // 100-107 (92+8=100, 92+15=107, bright colors)
                            } else if idx >= 40 && idx <= 47 {
                                sgr.push(format!("{}", idx)); // Direct ANSI codes 40-47
                            } else if idx >= 100 && idx <= 107 {
                                sgr.push(format!("{}", idx)); // Direct ANSI codes 100-107
                            } else {
                                sgr.push("49".to_string()); // default background
                            }
                        }
                        ColorProfileKind::ANSI256 => {
                            sgr.push(format!("48;5;{}", idx))
                        }
                        ColorProfileKind::NoColor => {}
                    }
                }
            }
        }

        // Final styling pass - "Layout First, Styling Second" approach
        let target_width = self.get_width();
        let target_height = self.get_height();
        let has_bg = self.is_set(BACKGROUND_KEY) || self.get_attr(ATTR_COLOR_WHITESPACE);

        // If no SGR codes and no width/height constraints, we're done.
        if sgr.is_empty() && target_width <= 0 && target_height <= 0 {
            return rendered;
        }

        let lines: Vec<&str> = rendered.split('\n').collect();
        let mut final_lines = Vec::with_capacity(lines.len());

        // LAYOUT FIRST: Create full-width canvas with alignment padding
        for line in lines {
            let mut canvas_line = line.to_string();
            
            // If a width is set, create full-width canvas with alignment padding
            if target_width > 0 {
                let line_vis_width = width_visible(&canvas_line);
                let gap = (target_width as usize).saturating_sub(line_vis_width);

                if gap > 0 {
                    let h_pos = self.get_align_horizontal().value();
                    let left_gap = (gap as f64 * h_pos).round() as usize;
                    let right_gap = gap - left_gap;

                    let left_pad = safe_repeat(' ', left_gap);
                    let right_pad = safe_repeat(' ', right_gap);
                    
                    // Create full-width canvas by adding alignment padding
                    canvas_line = format!("{}{}{}", left_pad, canvas_line, right_pad);
                }
            }
            
            final_lines.push(canvas_line);
        }

        // Height constraint with vertical alignment (integrated into Layout First phase)
        if target_height > 0 && (final_lines.len() as i32) < target_height {
            let gap = target_height as usize - final_lines.len();
            let v_pos = self.get_align_vertical().value();
            
            // Distribute padding lines based on vertical alignment
            // v_pos: 0.0=TOP (content at top, padding at bottom), 0.5=CENTER, 1.0=BOTTOM (content at bottom, padding at top)
            let top_pad_count = (gap as f64 * v_pos).round() as usize;
            let bottom_pad_count = gap - top_pad_count;
            
            // Determine width for padding lines to match existing canvas width
            let block_width = final_lines.iter().map(|l| width_visible(l)).max().unwrap_or(0);
            let empty_line = safe_repeat(' ', block_width);
            
            let mut height_adjusted = Vec::new();
            height_adjusted.extend(vec![empty_line.clone(); top_pad_count]);
            height_adjusted.extend(final_lines);
            height_adjusted.extend(vec![empty_line; bottom_pad_count]);
            
            final_lines = height_adjusted;
        }

        // Apply borders after layout constraints have been applied
        let render_borders = (self.get_border_top()
            || self.get_border_right()
            || self.get_border_bottom()
            || self.get_border_left())
            && self.is_set(BORDER_STYLE_KEY);
        if render_borders {
            let b = self.get_border_style();
            // Compute target width from the maximum visible width across all lines
            let mut w: usize = 0;
            for l in &final_lines {
                w = w.max(width_visible(l));
            }
            // Determine effective renderer/profile
            let eff = self.r.clone().unwrap_or_else(|| default_renderer().clone());
            let profile = eff.color_profile();

            // Helper to build SGR for a side using per-side then combined tokens
            let edge_sgr = |fg_opt: &Option<String>,
                            bg_opt: &Option<String>,
                            fg_combined: &Option<String>,
                            bg_combined: &Option<String>|
             -> String {
                if matches!(profile, ColorProfileKind::NoColor) {
                    return String::new();
                }
                let fg = fg_opt.as_ref().or(fg_combined.as_ref());
                let bg = bg_opt.as_ref().or(bg_combined.as_ref());
                let mut parts: Vec<String> = Vec::new();
                if let Some(tok) = fg {
                    if tok.starts_with('#') {
                        if let Some((r, g, b, _)) = parse_hex_rgba(tok) {
                            // RGB values are already 8-bit (0-255) cast to u32
                            parts.push(format!("38;2;{};{};{}", r, g, b));
                        } else {
                            // fallback for ANSI profiles
                            parts.push("38;5;0".to_string());
                        }
                    } else if let Ok(idx) = tok.parse::<u32>() {
                        let idx = idx % 256;
                        match profile {
                            ColorProfileKind::ANSI => {
                                if idx <= 7 {
                                    parts.push(format!("{}", 30 + idx)); // 30-37
                                } else if idx <= 15 {
                                    parts.push(format!("{}", 82 + idx)); // 90-97
                                } else if idx >= 30 && idx <= 37 {
                                    parts.push(format!("{}", idx)); // Direct ANSI codes 30-37
                                } else if idx >= 90 && idx <= 97 {
                                    parts.push(format!("{}", idx)); // Direct ANSI codes 90-97
                                } else {
                                    parts.push("39".to_string()); // default
                                }
                            }
                            _ => parts.push(format!("38;5;{}", idx)),
                        }
                    }
                }
                if let Some(tok) = bg {
                    if tok.starts_with('#') {
                        if let Some((r, g, b, _)) = parse_hex_rgba(tok) {
                            // RGB values are already 8-bit (0-255) cast to u32
                            parts.push(format!("48;2;{};{};{}", r, g, b));
                        } else {
                            parts.push("48;5;0".to_string());
                        }
                    } else if let Ok(idx) = tok.parse::<u32>() {
                        let idx = idx % 256;
                        match profile {
                            ColorProfileKind::ANSI => {
                                if idx <= 7 {
                                    parts.push(format!("{}", 40 + idx)); // 40-47
                                } else if idx <= 15 {
                                    parts.push(format!("{}", 92 + idx)); // 100-107
                                } else if idx >= 40 && idx <= 47 {
                                    parts.push(format!("{}", idx)); // Direct ANSI codes 40-47
                                } else if idx >= 100 && idx <= 107 {
                                    parts.push(format!("{}", idx)); // Direct ANSI codes 100-107
                                } else {
                                    parts.push("49".to_string()); // default
                                }
                            }
                            _ => parts.push(format!("48;5;{}", idx)),
                        }
                    }
                }
                if parts.is_empty() {
                    String::new()
                } else {
                    format!("\x1b[{}m", parts.join(";"))
                }
            };

            // Use stored combined fields if set via colors.rs helpers
            let combined_fg = self.border_top_fg_color.is_some()
                || self.border_right_fg_color.is_some()
                || self.border_bottom_fg_color.is_some()
                || self.border_left_fg_color.is_some();
            let combined_bg = self.border_top_bg_color.is_some()
                || self.border_right_bg_color.is_some()
                || self.border_bottom_bg_color.is_some()
                || self.border_left_bg_color.is_some();
            let fg_combined_ref = if combined_fg {
                None
            } else {
                self.fg_color.as_ref()
            };
            let bg_combined_ref = if combined_bg {
                None
            } else {
                self.bg_color.as_ref()
            };

            let top_sgr = edge_sgr(
                &self.border_top_fg_color,
                &self.border_top_bg_color,
                &fg_combined_ref.cloned(),
                &bg_combined_ref.cloned(),
            );
            let right_sgr = edge_sgr(
                &self.border_right_fg_color,
                &self.border_right_bg_color,
                &fg_combined_ref.cloned(),
                &bg_combined_ref.cloned(),
            );
            let bottom_sgr = edge_sgr(
                &self.border_bottom_fg_color,
                &self.border_bottom_bg_color,
                &fg_combined_ref.cloned(),
                &bg_combined_ref.cloned(),
            );
            let left_sgr = edge_sgr(
                &self.border_left_fg_color,
                &self.border_left_bg_color,
                &fg_combined_ref.cloned(),
                &bg_combined_ref.cloned(),
            );
            let reset = "\x1b[0m";

            // Build top border (conditionally)
            let top = if self.get_border_top() {
                if top_sgr.is_empty() {
                    format!(
                        "{}{}{}",
                        if self.get_border_left() { b.top_left } else { b.top },
                        safe_str_repeat(b.top, w),
                        if self.get_border_right() { b.top_right } else { b.top }
                    )
                } else {
                    format!(
                        "{}{}{}{}{}",
                        top_sgr,
                        if self.get_border_left() { b.top_left } else { b.top },
                        safe_str_repeat(b.top, w),
                        if self.get_border_right() { b.top_right } else { b.top },
                        reset
                    )
                }
            } else {
                String::new()
            };

            // Add left/right borders per line, padding each line to the max width
            let mid = {
                let mut out_lines: Vec<String> = Vec::with_capacity(final_lines.len());
                let left_part_base = if self.get_border_left() {
                    if left_sgr.is_empty() {
                        b.left.to_string()
                    } else {
                        format!("{}{}{}", left_sgr, b.left, reset)
                    }
                } else {
                    String::new()
                };
                let right_part_base = if self.get_border_right() {
                    if right_sgr.is_empty() {
                        b.right.to_string()
                    } else {
                        format!("{}{}{}", right_sgr, b.right, reset)
                    }
                } else {
                    String::new()
                };
                for l in &final_lines {
                    let lw = width_visible(l);
                    let pad = w.saturating_sub(lw);
                    let mut line_buf = String::with_capacity(w + 4);
                    line_buf.push_str(&left_part_base);
                    line_buf.push_str(l);
                    if pad > 0 {
                        line_buf.push_str(&safe_repeat(' ', pad));
                    }
                    line_buf.push_str(&right_part_base);
                    out_lines.push(line_buf);
                }
                out_lines.join("\n")
            };

            // Build bottom border (conditionally)
            let bot = if self.get_border_bottom() {
                if bottom_sgr.is_empty() {
                    format!(
                        "{}{}{}",
                        if self.get_border_left() { b.bottom_left } else { b.bottom },
                        safe_str_repeat(b.bottom, w),
                        if self.get_border_right() { b.bottom_right } else { b.bottom }
                    )
                } else {
                    format!(
                        "{}{}{}{}{}",
                        bottom_sgr,
                        if self.get_border_left() { b.bottom_left } else { b.bottom },
                        safe_str_repeat(b.bottom, w),
                        if self.get_border_right() { b.bottom_right } else { b.bottom },
                        reset
                    )
                }
            } else {
                String::new()
            };

            // Combine the parts and update final_lines with bordered content
            let bordered_content = if !top.is_empty() && !bot.is_empty() {
                format!("{}\n{}\n{}", top, mid, bot)
            } else if !top.is_empty() {
                format!("{}\n{}", top, mid)
            } else if !bot.is_empty() {
                format!("{}\n{}", mid, bot)
            } else {
                mid
            };
            
            // Replace final_lines with the bordered content
            final_lines = bordered_content.split('\n').map(|s| s.to_string()).collect();
        }

        // STYLING SECOND: Apply styling to entire canvas
        if !sgr.is_empty() {
            let prefix = format!("\x1b[{}m", sgr.join(";"));
            let suffix = "\x1b[0m";
            
            if has_bg {
                // For background colors, style the entire canvas including whitespace
                final_lines = final_lines.into_iter()
                    .map(|line| format!("{}{}{}", prefix, line, suffix))
                    .collect();
            } else {
                // For foreground-only styling, style only non-whitespace parts
                final_lines = final_lines.into_iter()
                    .map(|line| {
                        let leading_spaces = line.chars().take_while(|&c| c == ' ').count();
                        let trailing_spaces = line.chars().rev().take_while(|&c| c == ' ').count();
                        let content_start = leading_spaces;
                        let content_end = line.len().saturating_sub(trailing_spaces);

                        if content_start >= content_end {
                            line
                        } else {
                            let lead = &line[..content_start];
                            let mid = &line[content_start..content_end];
                            let trail = &line[content_end..];
                            format!("{}{}{}{}{}", lead, prefix, mid, suffix, trail)
                        }
                    })
                    .collect();
            }
        }

        let mut result = final_lines.join("\n");


        // Apply all margins as final step (matches Go implementation)
        result = self.apply_margins(&result);
        
        result
    }

    /// Apply margins to a fully-rendered block, using margin background color if set.
    /// This matches the Go implementation's applyMargins function.
    fn apply_margins(&self, block: &str) -> String {
        let top_margin = if self.is_set(MARGIN_TOP_KEY) {
            self.get_margin_top().max(0) as usize
        } else {
            0
        };
        let right_margin = if self.is_set(MARGIN_RIGHT_KEY) {
            self.get_margin_right().max(0) as usize
        } else {
            0
        };
        let bottom_margin = if self.is_set(MARGIN_BOTTOM_KEY) {
            self.get_margin_bottom().max(0) as usize
        } else {
            0
        };
        let left_margin = if self.is_set(MARGIN_LEFT_KEY) {
            self.get_margin_left().max(0) as usize
        } else {
            0
        };

        if top_margin == 0 && right_margin == 0 && bottom_margin == 0 && left_margin == 0 {
            return block.to_string();
        }

        // Determine margin background color
        // In Go: if marginBgColor is not set, margin is transparent
        // But to match the visual output, margin inherits from background if no explicit margin_background
        let margin_bg_color = if self.is_set(MARGIN_BACKGROUND_KEY) {
            self.get_margin_background()
        } else if self.is_set(BACKGROUND_KEY) {
            // This is the key insight: inherit from main background
            self.get_background()
        } else {
            None
        };

        let mut margin_style = Style::new();
        if let Some(bg) = margin_bg_color {
            margin_style = margin_style.background(bg);
        }

        // Apply left and right margins to each line
        let lines: Vec<String> = block.split('\n').map(|line| {
            let left = if left_margin > 0 {
                margin_style.render(&safe_repeat(' ', left_margin))
            } else {
                String::new()
            };
            let right = if right_margin > 0 {
                margin_style.render(&safe_repeat(' ', right_margin))
            } else {
                String::new()
            };
            format!("{}{}{}", left, line, right)
        }).collect();

        let mut result = Vec::new();

        // Apply top margin
        if top_margin > 0 {
            let block_width = lines.iter().map(|l| width_visible(l)).max().unwrap_or(0);
            let empty_line = if block_width > 0 {
                margin_style.render(&safe_repeat(' ', block_width))
            } else {
                String::new()
            };
            for _ in 0..top_margin {
                result.push(empty_line.clone());
            }
        }

        result.extend(lines);

        // Apply bottom margin
        if bottom_margin > 0 {
            let block_width = result.iter().map(|l| width_visible(l)).max().unwrap_or(0);
            let empty_line = if block_width > 0 {
                margin_style.render(&safe_repeat(' ', block_width))
            } else {
                String::new()
            };
            for _ in 0..bottom_margin {
                result.push(empty_line.clone());
            }
        }

        result.join("\n")
    }

    /// Applies this style to a string as a convenience wrapper around `render()`.
    ///
    /// This method is a direct alias for [`render()`](Self::render) and provides
    /// the same functionality with a more concise name for common usage patterns.
    ///
    /// # Arguments
    ///
    /// * `s` - The text content to style
    ///
    /// # Returns
    ///
    /// A string containing the input text with all style properties applied,
    /// wrapped with appropriate ANSI escape sequences for terminal display.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lipgloss::Style;
    ///
    /// let style = Style::default();
    ///
    /// // These two calls are equivalent:
    /// let output1 = style.apply("Warning!");
    /// let output2 = style.render("Warning!");
    /// assert_eq!(output1, output2);
    /// ```
    pub fn apply(&self, s: &str) -> String {
        self.render(s)
    }
}
