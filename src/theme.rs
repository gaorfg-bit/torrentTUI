//! Built-in colour themes.
//!
//! A set of popular dark colour schemes, bundled as ready-to-use presets. Each
//! entry carries a packed `0xRRGGBB` palette — background, text, the semantic
//! states and a muted ink — which [`Palette::theme`] maps onto the semantic
//! slots below.
//!
//! `system` reproduces TorrentTUI's original ANSI colours exactly, so it is the
//! default and a first launch is unchanged.
//!
//! A [`Theme`] is a flat set of semantic slots. Renderers never name a raw
//! `Color` themselves, so switching themes re-colours every widget at once.

use std::sync::LazyLock;

use ratatui::style::Color;

/// Semantic colour slots shared by every widget.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Theme {
    /// Borders, prompts, the active tab, the info hash — the UI's accent.
    pub primary: Color,
    /// Secondary accent: the upload speed, the "fetching metadata" state.
    pub accent: Color,
    /// Download speed, seeding, healthy state.
    pub success: Color,
    /// Informational messages, paused, capped.
    pub warning: Color,
    /// Errors, stalled/blocked, destructive actions.
    pub error: Color,
    /// Downloading state.
    pub info: Color,
    /// Primary text.
    pub text: Color,
    /// Secondary text: status-bar hints, the ratio.
    pub text_dim: Color,
    /// Labels, borders, placeholders — the quietest ink.
    pub muted: Color,
    /// Background of a marked row (the one place the UI paints a background).
    pub bg_element: Color,
    /// Progress-column ramp: `<25`, `25-50`, `50-75`, `75-100`, `100`.
    pub progress: [Color; 5],
}

impl Theme {
    /// The default theme: TorrentTUI's original ANSI palette, byte for byte.
    /// Kept hand-written (not derived from the presets) so the default look is
    /// exactly what the app showed before theming.
    pub const fn system() -> Theme {
        Theme {
            primary: Color::Cyan,
            accent: Color::Magenta,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Blue,
            text: Color::White,
            text_dim: Color::Gray,
            muted: Color::DarkGray,
            bg_element: Color::Indexed(236),
            progress: [
                Color::Red,
                Color::Rgb(255, 165, 0),
                Color::Yellow,
                Color::LightGreen,
                Color::Green,
            ],
        }
    }

    /// The progress-column colour for a completion percentage. Thresholds match
    /// the pre-theming gradient, and `system` reproduces it exactly.
    pub fn progress_color(&self, percent: f64) -> Color {
        if percent >= 100.0 {
            self.progress[4]
        } else if percent >= 75.0 {
            self.progress[3]
        } else if percent >= 50.0 {
            self.progress[2]
        } else if percent >= 25.0 {
            self.progress[1]
        } else {
            self.progress[0]
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::system()
    }
}

/// Expand a packed `0xRRGGBB` literal into a ratatui colour.
const fn rgb(hex: u32) -> Color {
    Color::Rgb(
        ((hex >> 16) & 0xFF) as u8,
        ((hex >> 8) & 0xFF) as u8,
        (hex & 0xFF) as u8,
    )
}

/// One channel of [`mix`].
const fn mix_channel(a: u32, b: u32, t: u32, shift: u32) -> u32 {
    let av = (a >> shift) & 0xFF;
    let bv = (b >> shift) & 0xFF;
    (av * (100 - t) + bv * t + 50) / 100
}

/// Blend two packed colours, `t` percent of the way from `a` to `b`, rounded.
/// Integer maths so it can run in a `const` context.
const fn mix(a: u32, b: u32, t: u32) -> u32 {
    (mix_channel(a, b, t, 16) << 16) | (mix_channel(a, b, t, 8) << 8) | mix_channel(a, b, t, 0)
}

/// A preset's palette, packed as
/// `[background, text, primary, accent, success, warning, error, info, muted]`.
struct Palette {
    id: &'static str,
    name: &'static str,
    hex: [u32; 9],
}

impl Palette {
    /// Derive the semantic slots. The two greys come straight from the palette
    /// (text, muted); one blend darkens the marked-row background and another
    /// builds the progress ramp, so every widget is covered without a
    /// per-theme override table.
    const fn theme(&self) -> Theme {
        let [neutral, ink, primary, accent, success, warning, error, info, comment] = self.hex;
        Theme {
            primary: rgb(primary),
            accent: rgb(accent),
            success: rgb(success),
            warning: rgb(warning),
            error: rgb(error),
            info: rgb(info),
            text: rgb(ink),
            text_dim: rgb(mix(ink, comment, 50)),
            muted: rgb(comment),
            bg_element: rgb(mix(neutral, ink, 6)),
            progress: [
                rgb(error),
                rgb(mix(error, warning, 50)),
                rgb(warning),
                rgb(mix(warning, success, 50)),
                rgb(success),
            ],
        }
    }
}

/// The built-in palettes, one line per theme — `rustfmt` is opted out to keep
/// them that way.
#[rustfmt::skip]
const PALETTES: &[Palette] = &[
    Palette { id: "amoled", name: "AMOLED", hex: [0x000000, 0xffffff, 0xb388ff, 0xff4081, 0x00ff88, 0xffea00, 0xff1744, 0x18ffff, 0x555555] },
    Palette { id: "aura", name: "Aura", hex: [0x15141b, 0xedecee, 0xa277ff, 0xff6767, 0x61ffca, 0xffca85, 0xff6767, 0x82e2ff, 0x6d6a7e] },
    Palette { id: "ayu", name: "Ayu", hex: [0x0f1419, 0xd6dae0, 0x3fb7e3, 0xf2856f, 0x78d05c, 0xe4a75c, 0xf58572, 0x66c6f1, 0x5a6673] },
    Palette { id: "carbonfox", name: "Carbonfox", hex: [0x393939, 0xf2f4f8, 0x33b1ff, 0xff8389, 0x42be65, 0xf1c21b, 0xff8389, 0x78a9ff, 0x6f6f6f] },
    Palette { id: "catppuccin", name: "Catppuccin", hex: [0x1e1e2e, 0xcdd6f4, 0xb4befe, 0xf38ba8, 0xa6d189, 0xf4b8e4, 0xf38ba8, 0x89dceb, 0x6c7086] },
    Palette { id: "catppuccin-frappe", name: "Catppuccin Frappe", hex: [0x303446, 0xc6d0f5, 0x8da4e2, 0xf4b8e4, 0xa6d189, 0xe5c890, 0xe78284, 0x81c8be, 0x949cb8] },
    Palette { id: "catppuccin-macchiato", name: "Catppuccin Macchiato", hex: [0x24273a, 0xcad3f5, 0x8aadf4, 0xf5bde6, 0xa6da95, 0xeed49f, 0xed8796, 0x8bd5ca, 0x939ab7] },
    Palette { id: "cobalt2", name: "Cobalt2", hex: [0x193549, 0xffffff, 0x0088ff, 0x2affdf, 0x9eff80, 0xffc600, 0xff0088, 0xff9d00, 0x0088ff] },
    Palette { id: "cursor", name: "Cursor", hex: [0x181818, 0xe4e4e4, 0x88c0d0, 0x88c0d0, 0x3fa266, 0xf1b467, 0xe34671, 0x81a1c1, 0xe4e4e4] },
    Palette { id: "dracula", name: "Dracula", hex: [0x1d1e28, 0xf8f8f2, 0xbd93f9, 0xff79c6, 0x50fa7b, 0xffb86c, 0xff5555, 0x8be9fd, 0x6272a4] },
    Palette { id: "everforest", name: "Everforest", hex: [0x2d353b, 0xd3c6aa, 0xa7c080, 0xd699b6, 0xa7c080, 0xe69875, 0xe67e80, 0x83c092, 0x7a8478] },
    Palette { id: "flexoki", name: "Flexoki", hex: [0x100f0f, 0xcecdc3, 0xda702c, 0x8b7ec8, 0x879a39, 0xda702c, 0xd14d41, 0x3aa99f, 0x6f6e69] },
    Palette { id: "github", name: "GitHub", hex: [0x0d1117, 0xc9d1d9, 0x58a6ff, 0x39c5cf, 0x3fb950, 0xe3b341, 0xf85149, 0xd29922, 0x8b949e] },
    Palette { id: "gruvbox", name: "Gruvbox", hex: [0x282828, 0xebdbb2, 0x83a598, 0xfb4934, 0xb8bb26, 0xfabd2f, 0xfb4934, 0xd3869b, 0x928374] },
    Palette { id: "kanagawa", name: "Kanagawa", hex: [0x1f1f28, 0xdcd7ba, 0x7e9cd8, 0xd27e99, 0x98bb6c, 0xd7a657, 0xe82424, 0x76946a, 0x727169] },
    Palette { id: "lucent-orng", name: "Lucent Orng", hex: [0x2a1a15, 0xeeeeee, 0xec5b2b, 0xfff7f1, 0x6ba1e6, 0xec5b2b, 0xe06c75, 0x56b6c2, 0x808080] },
    Palette { id: "material", name: "Material", hex: [0x263238, 0xeeffff, 0x82aaff, 0x89ddff, 0xc3e88d, 0xffcb6b, 0xf07178, 0xffcb6b, 0x546e7a] },
    Palette { id: "matrix", name: "Matrix", hex: [0x0a0e0a, 0x62ff94, 0x2eff6a, 0xc770ff, 0x62ff94, 0xe6ff57, 0xff4b4b, 0x30b3ff, 0x8ca391] },
    Palette { id: "mercury", name: "Mercury", hex: [0x171721, 0xdddde5, 0x8da4f5, 0x8da4f5, 0x77c599, 0xfc9b6f, 0xfc92b4, 0x77becf, 0x9d9da8] },
    Palette { id: "monokai", name: "Monokai", hex: [0x272822, 0xf8f8f2, 0xae81ff, 0xf92672, 0xa6e22e, 0xfd971f, 0xf92672, 0x66d9ef, 0x75715e] },
    Palette { id: "nightowl", name: "Night Owl", hex: [0x011627, 0xd6deeb, 0x82aaff, 0xf78c6c, 0xc5e478, 0xecc48d, 0xef5350, 0x82aaff, 0x637777] },
    Palette { id: "nord", name: "Nord", hex: [0x2e3440, 0xe5e9f0, 0x88c0d0, 0xd57780, 0xa3be8c, 0xd08770, 0xbf616a, 0x81a1c1, 0x616e88] },
    Palette { id: "one-dark", name: "One Dark", hex: [0x282c34, 0xabb2bf, 0x61afef, 0x56b6c2, 0x98c379, 0xe5c07b, 0xe06c75, 0xd19a66, 0x5c6370] },
    Palette { id: "onedarkpro", name: "One Dark Pro", hex: [0x1e222a, 0xabb2bf, 0x61afef, 0xe06c75, 0x98c379, 0xe5c07b, 0xe06c75, 0x56b6c2, 0x5c6370] },
    Palette { id: "orng", name: "Orng", hex: [0x0a0a0a, 0xeeeeee, 0xec5b2b, 0xfff7f1, 0x6ba1e6, 0xec5b2b, 0xe06c75, 0x56b6c2, 0x808080] },
    Palette { id: "osaka-jade", name: "Osaka Jade", hex: [0x111c18, 0xc1c497, 0x2dd5b7, 0x549e6a, 0x549e6a, 0xe5c736, 0xff5345, 0x2dd5b7, 0x53685b] },
    Palette { id: "palenight", name: "Palenight", hex: [0x292d3e, 0xa6accd, 0x82aaff, 0x89ddff, 0xc3e88d, 0xffcb6b, 0xf07178, 0xf78c6c, 0x676e95] },
    Palette { id: "rosepine", name: "Rose Pine", hex: [0x191724, 0xe0def4, 0x9ccfd8, 0xebbcba, 0x31748f, 0xf6c177, 0xeb6f92, 0x9ccfd8, 0x6e6a86] },
    Palette { id: "shadesofpurple", name: "Shades of Purple", hex: [0x1a102b, 0xf5f0ff, 0xc792ff, 0xff7ac6, 0x7be0b0, 0xffd580, 0xff7ac6, 0x7dd4ff, 0xb362ff] },
    Palette { id: "solarized", name: "Solarized", hex: [0x002b36, 0x93a1a1, 0x6c71c4, 0xd33682, 0x859900, 0xb58900, 0xdc322f, 0x2aa198, 0x586e75] },
    Palette { id: "synthwave84", name: "Synthwave '84", hex: [0x262335, 0xffffff, 0x36f9f6, 0xb084eb, 0x72f1b8, 0xfede5d, 0xfe4450, 0xff8b39, 0x848bbd] },
    Palette { id: "tokyonight", name: "Tokyonight", hex: [0x1a1b26, 0xc0caf5, 0x7aa2f7, 0xff9e64, 0x9ece6a, 0xe0af68, 0xf7768e, 0x7dcfff, 0x565f89] },
    Palette { id: "vercel", name: "Vercel", hex: [0x000000, 0xededed, 0x0070f3, 0x8e4ec6, 0x46a758, 0xffb224, 0xe5484d, 0x52a8ff, 0x878787] },
    Palette { id: "vesper", name: "Vesper", hex: [0x101010, 0xffffff, 0xffc799, 0xff8080, 0x99ffe4, 0xffc799, 0xff8080, 0xffc799, 0x8b8b8b] },
    Palette { id: "zenburn", name: "Zenburn", hex: [0x3f3f3f, 0xdcdccc, 0x8cd0d3, 0x93e0e3, 0x7f9f7f, 0xf0dfaf, 0xcc9393, 0xdfaf8f, 0x7f9f7f] },
];

/// One selectable theme: a stable `id` (written to `config.toml`), a display
/// `name`, and its palette.
pub struct ThemeEntry {
    pub id: &'static str,
    pub name: &'static str,
    pub theme: Theme,
}

/// The theme registry: `system` first (the default), then the built-in
/// palettes. Built once on first use.
pub static THEMES: LazyLock<Vec<ThemeEntry>> = LazyLock::new(|| {
    let mut themes = Vec::with_capacity(PALETTES.len() + 1);
    themes.push(ThemeEntry {
        id: "system",
        name: "System",
        theme: Theme::system(),
    });
    themes.extend(PALETTES.iter().map(|p| ThemeEntry {
        id: p.id,
        name: p.name,
        theme: p.theme(),
    }));
    themes
});

/// Resolve a theme id — typically a `[ui] theme` value — to its entry. An
/// unknown id falls back to `system`, so a stale or mistyped config value can
/// never stop the app from starting.
pub fn by_name(name: &str) -> &'static ThemeEntry {
    THEMES.iter().find(|e| e.id == name).unwrap_or(&THEMES[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_ids_are_unique_and_non_empty() {
        for (i, a) in THEMES.iter().enumerate() {
            assert!(!a.id.is_empty(), "empty id");
            assert!(!a.name.is_empty(), "empty name for {}", a.id);
            assert!(
                !THEMES[..i].iter().any(|b| b.id == a.id),
                "duplicate id {}",
                a.id
            );
        }
    }

    #[test]
    fn the_registry_has_system_first_then_the_built_in_palettes() {
        assert_eq!(THEMES[0].id, "system");
        assert!(THEMES.len() > 1, "the built-in palettes must be present");
        assert!(THEMES.iter().any(|e| e.id == "gruvbox"));
    }

    #[test]
    fn unknown_name_falls_back_to_system() {
        assert_eq!(by_name("system").id, "system");
        assert_eq!(by_name("no-such-theme").id, "system");
        assert_eq!(by_name("").id, "system");
    }

    #[test]
    fn a_built_in_palette_differs_from_the_system_default() {
        // Guards against an expanded table collapsing to black, i.e. every
        // preset ending up identical to `system`.
        assert_ne!(by_name("gruvbox").theme, Theme::system());
    }

    #[test]
    fn system_progress_ramp_matches_the_original_gradient() {
        let t = Theme::system();
        assert_eq!(t.progress_color(0.0), Color::Red);
        assert_eq!(t.progress_color(24.9), Color::Red);
        assert_eq!(t.progress_color(25.0), Color::Rgb(255, 165, 0));
        assert_eq!(t.progress_color(49.9), Color::Rgb(255, 165, 0));
        assert_eq!(t.progress_color(50.0), Color::Yellow);
        assert_eq!(t.progress_color(74.9), Color::Yellow);
        assert_eq!(t.progress_color(75.0), Color::LightGreen);
        assert_eq!(t.progress_color(99.9), Color::LightGreen);
        assert_eq!(t.progress_color(100.0), Color::Green);
    }
}
