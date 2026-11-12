//! Design tokens (four UI brightness levels plus the Al Majlis light and dark looks), fonts and
//! egui style.

use std::sync::Arc;

use egui::{Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Stroke, TextStyle, Visuals};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Brightness {
    Dark,
    MediumDark,
    MediumLight,
    Light,
    /// Al Majlis light (docs/craft/MAJLIS_LOOK.md in Al Majlis), the default: its own colours and
    /// shapes, whatever accent or colour scheme is chosen.
    #[default]
    MajlisLight,
    /// Al Majlis dark.
    MajlisDark,
}

impl Brightness {
    pub const ALL: [Brightness; 6] = [
        Brightness::MajlisLight,
        Brightness::MajlisDark,
        Brightness::Dark,
        Brightness::MediumDark,
        Brightness::MediumLight,
        Brightness::Light,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Brightness::Dark => "Dark",
            Brightness::MediumDark => "Medium Dark",
            Brightness::MediumLight => "Medium Light",
            Brightness::Light => "Light",
            Brightness::MajlisLight => "Al Majlis Light",
            Brightness::MajlisDark => "Al Majlis Dark",
        }
    }
    pub fn id(self) -> &'static str {
        match self {
            Brightness::Dark => "dark",
            Brightness::MediumDark => "mediumDark",
            Brightness::MediumLight => "mediumLight",
            Brightness::Light => "light",
            Brightness::MajlisLight => "majlisLight",
            Brightness::MajlisDark => "majlisDark",
        }
    }
    /// One of the Al Majlis looks.
    pub fn is_majlis(self) -> bool {
        matches!(self, Brightness::MajlisLight | Brightness::MajlisDark)
    }
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|b| b.id().eq_ignore_ascii_case(s) || b.label().eq_ignore_ascii_case(s))
    }
}

/// User Interface › Accent Color: the one colour for what is on, chosen or focused. Coral, the
/// default, is the app icon's.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Accent {
    #[default]
    Coral,
    Blue,
    Violet,
    Green,
    Amber,
    Pink,
}

impl Accent {
    pub const ALL: [Accent; 6] = [Accent::Coral, Accent::Blue, Accent::Violet, Accent::Green, Accent::Amber, Accent::Pink];
    pub fn id(self) -> &'static str {
        match self {
            Accent::Coral => "coral",
            Accent::Blue => "blue",
            Accent::Violet => "violet",
            Accent::Green => "green",
            Accent::Amber => "amber",
            Accent::Pink => "pink",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|a| a.id().eq_ignore_ascii_case(s))
    }
    /// (accent, accent_strong) on dark panels and on light ones; the strong one fills buttons
    /// with white lettering.
    fn colors(self, dark: bool) -> (u32, u32) {
        match (self, dark) {
            (Accent::Coral, true) => (0xf06a52, 0xe8573f),
            (Accent::Coral, false) => (0xd9472f, 0xd14a33),
            (Accent::Blue, true) => (0x5b96ff, 0x2f74f0),
            (Accent::Blue, false) => (0x2563eb, 0x1f5ad6),
            (Accent::Violet, true) => (0xa98bff, 0x7c55f0),
            (Accent::Violet, false) => (0x7146dc, 0x6a3fd4),
            (Accent::Green, true) => (0x3fcf8e, 0x1f9d63),
            (Accent::Green, false) => (0x16955a, 0x14874f),
            (Accent::Amber, true) => (0xf5b544, 0xc47f0e),
            (Accent::Amber, false) => (0xb06d06, 0xb06d06),
            (Accent::Pink, true) => (0xf36aa3, 0xdb3f7c),
            (Accent::Pink, false) => (0xcc3672, 0xc22e69),
        }
    }
}

/// User Interface › Color Scheme: Modern (the default: slate greys and the chosen accent) or
/// Classic (the neutral greys and blue accent of earlier versions). Both keep the same layout,
/// cards and corners; only the colours differ.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Scheme {
    #[default]
    Modern,
    Classic,
}

impl Scheme {
    pub const ALL: [Scheme; 2] = [Scheme::Modern, Scheme::Classic];
    pub fn id(self) -> &'static str {
        match self {
            Scheme::Modern => "modern",
            Scheme::Classic => "classic",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|c| c.id().eq_ignore_ascii_case(s))
    }
}

/// All colours and metrics the UI uses. Never hard-code a colour in a widget.
#[derive(Clone, Copy, Debug)]
pub struct Tokens {
    pub dark: bool,
    pub app_bar: Color32,
    pub panel: Color32,
    pub panel_darker: Color32,
    pub input: Color32,
    pub input_border: Color32,
    pub border: Color32,
    pub divider: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub text_disabled: Color32,
    pub icon: Color32,
    pub hover: Color32,
    pub tool_active: Color32,
    pub accent: Color32,
    pub accent_strong: Color32,
    pub row_selected: Color32,
    pub pasteboard: Color32,
    pub ruler: Color32,
    pub ruler_tick: Color32,
    pub smart_guide: Color32,
    pub guide: Color32,
    pub measure_bg: Color32,
    pub button: Color32,
    /// Corner radius of egui's own widgets (fields, combo boxes, egui buttons, scroll bars).
    pub radius: u8,
    /// The Al Majlis look ([`Brightness::is_majlis`]): pill tabs and text buttons, hairlines.
    pub majlis: bool,
    /// Corner radius of tool wells and icon buttons.
    pub tool_radius: u8,
    /// Corner radius of the app's text buttons (primary, secondary, flat); 255 = pill.
    pub button_radius: u8,
    /// Corner radius of chips, crumbs, small toggles and tab tracks; 255 = pill.
    pub pill_radius: u8,
    /// Corner radius of the app's own fields (numbers, text, dropdowns).
    pub field_radius: u8,
    /// Corner radius of cards and panels ([`card`]).
    pub card_radius: u8,
    /// Corner radius of windows, dialogs and popovers.
    pub window_radius: u8,
    /// Corner radius of menus.
    pub menu_radius: u8,
    /// Raised surface (hover, header, field).
    pub raised: Color32,
    /// Pressed or active control.
    pub pressed: Color32,
    /// 1 px outline on a panel surface.
    pub hairline: Color32,
    /// 1 px outline on the window background.
    pub hairline_canvas: Color32,
    /// Muted text (inactive tabs).
    pub text_muted: Color32,
    /// Accent under the pointer or pressed (primary buttons).
    pub accent_hover: Color32,
    /// Small text in the accent colour.
    pub accent_text: Color32,
    /// Soft accent fill (selected tool, selected rows).
    pub accent_soft: Color32,
    /// Keyboard focus ring.
    pub focus_ring: Color32,
    /// Text and icons on an accent fill.
    pub on_accent: Color32,
    /// A slider's rail.
    pub rail: Color32,
    /// Success and information text.
    pub ok: Color32,
    pub info: Color32,
    /// Field values, button labels, link labels.
    pub text_strong: Color32,
    /// Panel tab strips, doc-tab bar, dock header.
    pub tab_strip: Color32,
    /// Outline of secondary (outlined) buttons.
    pub button_border: Color32,
    /// Default Layer 1 selection colour and selected-anchor fill.
    pub selection: Color32,
    pub anchor_selected: Color32,
    /// Active item text in tool flyouts.
    pub flyout_active: Color32,
    /// Hint-bar / tab-bar heights etc. scale with this base font size.
    pub font_size: f32,
    /// Window Close button (custom title bar) when hovered, and its glyph there.
    pub caption_close: Color32,
    pub caption_close_text: Color32,
    /// What preview images mark (Flattener Preview highlights).
    pub highlight: Color32,
    /// The bleed outline around artboards (Document Setup → Bleed).
    pub bleed: Color32,
    /// An artboard's Show Center Mark and Show Cross Hairs (Artboard Options), drawn over its
    /// paper in every brightness.
    pub artboard_mark: Color32,
    /// Error marks (a missing linked file).
    pub error: Color32,
    /// Warning marks (a modified linked file).
    pub warning: Color32,
    /// Rotation gizmo axes, also labelled X, Y and Z.
    pub axis_x: Color32,
    pub axis_y: Color32,
    pub axis_z: Color32,
}

impl Tokens {
    pub fn for_brightness(b: Brightness) -> Self {
        let hex = |s: u32| Color32::from_rgb((s >> 16) as u8, (s >> 8) as u8, s as u8);
        // The Craft look: cool slate greys, flat faces with soft corners, and one accent (the app
        // icon's coral) for what is on, chosen or focused. The four brightness levels share the
        // style and differ only in their greys.
        let base = Tokens {
            dark: true,
            app_bar: hex(0x0b0d12),
            panel: hex(0x181b22),
            panel_darker: hex(0x13161c),
            input: hex(0x0f1116),
            input_border: hex(0x2c323e),
            border: hex(0x06070a),
            divider: hex(0x232832),
            text: hex(0xdfe4ec),
            text_dim: hex(0x969faf),
            text_disabled: hex(0x5a6271),
            icon: hex(0xc9d0dc),
            hover: hex(0x262b35),
            tool_active: hex(0x2f3541),
            accent: hex(0xf06a52),
            accent_strong: hex(0xe8573f),
            row_selected: hex(0x3a2a2c),
            pasteboard: hex(0x101217),
            ruler: hex(0x13161c),
            ruler_tick: hex(0x6f788a),
            smart_guide: hex(0xff3dfc),
            guide: hex(0x4affff),
            measure_bg: Color32::from_rgba_unmultiplied(24, 27, 34, 235),
            button: hex(0x282d37),
            radius: 4,
            majlis: false,
            tool_radius: 6,
            button_radius: 6,
            pill_radius: 3,
            field_radius: 3,
            card_radius: CARD_RADIUS,
            window_radius: 10,
            menu_radius: 8,
            // The colours below follow the others outside Al Majlis (`Tokens::legacy_extras`).
            raised: Color32::TRANSPARENT,
            pressed: Color32::TRANSPARENT,
            hairline: Color32::TRANSPARENT,
            hairline_canvas: Color32::TRANSPARENT,
            text_muted: Color32::TRANSPARENT,
            accent_hover: Color32::TRANSPARENT,
            accent_text: Color32::TRANSPARENT,
            accent_soft: Color32::TRANSPARENT,
            focus_ring: Color32::TRANSPARENT,
            on_accent: Color32::WHITE,
            rail: Color32::TRANSPARENT,
            ok: hex(0x3fcf8e),
            info: hex(0x5b96ff),
            text_strong: hex(0xffffff),
            tab_strip: hex(0x0f1116),
            button_border: hex(0x4a5363),
            selection: hex(0x4f80ff),
            anchor_selected: hex(0x3d82ff),
            flyout_active: hex(0xff8a74),
            font_size: 13.0,
            caption_close: hex(0xc42b1c),
            caption_close_text: Color32::WHITE,
            highlight: hex(0xff3030),
            bleed: hex(0xf03030),
            artboard_mark: hex(0x808080),
            error: hex(0xe34850),
            warning: hex(0xf0a330),
            axis_x: hex(0xef6058),
            axis_y: hex(0x45af63),
            axis_z: hex(0x378ef0),
        };
        let legacy = match b {
            Brightness::MajlisLight | Brightness::MajlisDark => return Self::majlis(b == Brightness::MajlisDark),
            Brightness::Dark => base,
            // Charcoal.
            Brightness::MediumDark => Tokens {
                app_bar: hex(0x1b1f27),
                panel: hex(0x272c36),
                panel_darker: hex(0x21262f),
                tab_strip: hex(0x1e222a),
                input: hex(0x1a1e26),
                input_border: hex(0x3a414e),
                border: hex(0x11141a),
                divider: hex(0x323946),
                text: hex(0xe4e9f1),
                text_dim: hex(0x99a2b2),
                text_disabled: hex(0x626b7a),
                icon: hex(0xccd3de),
                hover: hex(0x353c49),
                tool_active: hex(0x3d4553),
                row_selected: hex(0x4a3236),
                pasteboard: hex(0x1e222a),
                ruler: hex(0x21262f),
                ruler_tick: hex(0x7c8597),
                measure_bg: Color32::from_rgba_unmultiplied(39, 44, 54, 235),
                button: hex(0x39404d),
                button_border: hex(0x5a6476),
                ..base
            },
            Brightness::MediumLight => Tokens {
                dark: false,
                app_bar: hex(0xc4cad4),
                panel: hex(0xd3d8e0),
                panel_darker: hex(0xc9cfd8),
                tab_strip: hex(0xc4cad4),
                input: hex(0xe9ecf1),
                input_border: hex(0x9aa2b0),
                border: hex(0xa9b0bc),
                divider: hex(0xbcc3ce),
                text: hex(0x1b1f27),
                text_strong: hex(0x000000),
                text_dim: hex(0x4a5160),
                text_disabled: hex(0x7d8594),
                icon: hex(0x272c36),
                hover: hex(0xc5ccd6),
                tool_active: hex(0xb6bec9),
                accent: hex(0xd9472f),
                accent_strong: hex(0xd14a33),
                flyout_active: hex(0xc23e28),
                row_selected: hex(0xe7c4bd),
                pasteboard: hex(0xb9c0cb),
                ruler: hex(0xccd2db),
                ruler_tick: hex(0x4a5160),
                measure_bg: Color32::from_rgba_unmultiplied(211, 216, 224, 235),
                button: hex(0xc9d0da),
                button_border: hex(0x8a93a2),
                ..base
            },
            Brightness::Light => Tokens {
                dark: false,
                app_bar: hex(0xe6e9ee),
                panel: hex(0xf3f5f8),
                panel_darker: hex(0xe9ecf1),
                tab_strip: hex(0xe6e9ee),
                input: hex(0xffffff),
                input_border: hex(0xc3c9d3),
                border: hex(0xd0d5dd),
                divider: hex(0xdde1e8),
                text: hex(0x1b1f27),
                text_strong: hex(0x000000),
                text_dim: hex(0x58606e),
                text_disabled: hex(0x9aa1ad),
                icon: hex(0x2a303b),
                hover: hex(0xe3e7ed),
                tool_active: hex(0xd5dbe3),
                accent: hex(0xd9472f),
                accent_strong: hex(0xd14a33),
                flyout_active: hex(0xc23e28),
                row_selected: hex(0xf8d9d2),
                pasteboard: hex(0xdfe3e9),
                ruler: hex(0xeceff3),
                ruler_tick: hex(0x58606e),
                measure_bg: Color32::from_rgba_unmultiplied(243, 245, 248, 235),
                button: hex(0xe3e7ed),
                button_border: hex(0xa4abb8),
                ..base
            },
        };
        legacy.legacy_extras()
    }

    /// The Al Majlis look (docs/craft/MAJLIS_LOOK.md in Al Majlis): the exact Al Majlis colours,
    /// rounded shapes, pill tabs and text buttons, 1 px hairlines. The canvas around the artboards
    /// keeps a neutral grey, so colours on the artboard are judged as before.
    pub fn majlis(dark: bool) -> Self {
        let hex = |s: u32| Color32::from_rgb((s >> 16) as u8, (s >> 8) as u8, s as u8);
        // (light, dark)
        let pick = |l: u32, d: u32| hex(if dark { d } else { l });
        let canvas = pick(0xF2F1EE, 0x0D0D0F);
        let surface = pick(0xFFFFFF, 0x17171A);
        let raised = pick(0xF6F5F3, 0x202024);
        let pressed = pick(0xE8E7E3, 0x2E2E34);
        let hairline = pick(0xEEEEEE, 0x29292B);
        let hairline_canvas = pick(0xE2E1DE, 0x202022);
        let text = pick(0x141416, 0xF4F4F2);
        let text_dim = pick(0x5B5B5C, 0xB2B2B1);
        let text_muted = pick(0x7E7E7F, 0x909091);
        let accent = hex(0x2563EB);
        let accent_hover = pick(0x1D4ED8, 0x3B82F6);
        let accent_text = pick(0x1D4ED8, 0x93C5FD);
        let accent_soft = pick(0xE9EFFD, 0x19233B);
        let focus_ring =
            if dark { Color32::from_rgba_unmultiplied(0x60, 0xA5, 0xFA, 128) } else { Color32::from_rgba_unmultiplied(0x25, 0x63, 0xEB, 115) };
        let measure_bg =
            if dark { Color32::from_rgba_unmultiplied(0x17, 0x17, 0x1A, 235) } else { Color32::from_rgba_unmultiplied(0xFF, 0xFF, 0xFF, 235) };
        let base = Self::for_brightness(if dark { Brightness::Dark } else { Brightness::Light });
        Tokens {
            dark,
            majlis: true,
            app_bar: canvas,
            panel: surface,
            panel_darker: raised,
            input: raised,
            input_border: hairline,
            border: hairline_canvas,
            divider: hairline,
            text,
            text_strong: text,
            text_dim,
            text_disabled: text_muted,
            icon: text,
            hover: raised,
            tool_active: accent_soft,
            accent,
            accent_strong: accent,
            row_selected: accent_soft,
            // Neutral greys, as Classic's: the canvas around the artboards.
            pasteboard: pick(0xDCDCDC, 0x1E1E1E),
            ruler: raised,
            ruler_tick: text_dim,
            measure_bg,
            button: raised,
            tab_strip: raised,
            button_border: hairline,
            flyout_active: accent_text,
            error: pick(0xB42318, 0xFDA29B),
            warning: pick(0xB54708, 0xFEC84B),
            ok: pick(0x067647, 0x75E0A7),
            info: pick(0x175CD3, 0x84CAFF),
            radius: 8,
            tool_radius: 8,
            button_radius: u8::MAX,
            pill_radius: u8::MAX,
            field_radius: 8,
            card_radius: 12,
            window_radius: 16,
            menu_radius: 12,
            raised,
            pressed,
            hairline,
            hairline_canvas,
            text_muted,
            accent_hover,
            accent_text,
            accent_soft,
            focus_ring,
            on_accent: Color32::WHITE,
            rail: pressed,
            ..base
        }
    }

    /// Outside Al Majlis, the Al Majlis-only colours follow the theme's own ones, so every widget
    /// that reads them looks as it did before they were added.
    fn legacy_extras(self) -> Self {
        Tokens {
            raised: self.hover,
            pressed: self.tool_active,
            hairline: self.divider,
            hairline_canvas: self.border,
            text_muted: self.text_dim,
            accent_hover: self.accent,
            accent_text: self.flyout_active,
            accent_soft: self.tool_active,
            focus_ring: self.accent,
            on_accent: Color32::WHITE,
            rail: self.button_border,
            ..self
        }
    }

    /// The Classic colour scheme at brightness `b`: neutral greys with a blue accent, a dark
    /// pressed tool and blue-grey selected rows, on the Modern metrics (corners, sizes).
    pub fn classic(b: Brightness) -> Self {
        let hex = |s: u32| Color32::from_rgb((s >> 16) as u8, (s >> 8) as u8, s as u8);
        if b.is_majlis() {
            return Self::for_brightness(b);
        }
        let base = Tokens {
            app_bar: hex(0x262626),
            panel: hex(0x323232),
            panel_darker: hex(0x282828),
            input: hex(0x1f1f1f),
            input_border: hex(0x464646),
            border: hex(0x1b1b1b),
            divider: hex(0x3f3f3f),
            text: hex(0xd8d8d8),
            text_dim: hex(0xa8a8a8),
            text_disabled: hex(0x6e6e6e),
            icon: hex(0xcfcfcf),
            hover: hex(0x444444),
            tool_active: hex(0x1d1d1d),
            accent: hex(0x378ef0),
            accent_strong: hex(0x1473e6),
            row_selected: hex(0x484b50),
            pasteboard: hex(0x1e1e1e),
            ruler: hex(0x2f2f2f),
            ruler_tick: hex(0x8a8a8a),
            measure_bg: Color32::from_rgba_unmultiplied(92, 92, 92, 235),
            button: hex(0x444444),
            text_strong: hex(0xf5f5f5),
            tab_strip: hex(0x282828),
            button_border: hex(0x6a6a6a),
            flyout_active: hex(0x54b2f7),
            ..Tokens::for_brightness(Brightness::Dark)
        };
        let classic = match b {
            Brightness::Dark | Brightness::MajlisLight | Brightness::MajlisDark => base,
            Brightness::MediumDark => Tokens {
                // A shade below the panels, so the cards stand out from the window behind them.
                app_bar: hex(0x464646),
                panel: hex(0x535353),
                panel_darker: hex(0x424242),
                tab_strip: hex(0x424242),
                input: hex(0x454545),
                input_border: hex(0x5f5f5f),
                border: hex(0x383838),
                divider: hex(0x4b4b4b),
                text: hex(0xd1d1d1),
                text_strong: hex(0xffffff),
                text_dim: hex(0xb0b0b0),
                text_disabled: hex(0x7a7a7a),
                icon: hex(0xc2c2c2),
                hover: hex(0x606060),
                tool_active: hex(0x303030),
                row_selected: hex(0x52677c),
                pasteboard: hex(0x606060),
                ruler: hex(0x333333),
                button: hex(0x535353),
                button_border: hex(0x747474),
                ..base
            },
            Brightness::MediumLight => Tokens {
                dark: false,
                app_bar: hex(0xa9a9a9),
                panel: hex(0xb8b8b8),
                panel_darker: hex(0xaaaaaa),
                tab_strip: hex(0xa6a6a6),
                input: hex(0xd6d6d6),
                input_border: hex(0x8c8c8c),
                border: hex(0x969696),
                divider: hex(0x9e9e9e),
                text: hex(0x1f1f1f),
                text_strong: hex(0x000000),
                text_dim: hex(0x3c3c3c),
                text_disabled: hex(0x7c7c7c),
                icon: hex(0x2a2a2a),
                hover: hex(0xc8c8c8),
                tool_active: hex(0x969696),
                row_selected: hex(0x9fb4d6),
                pasteboard: hex(0xa0a0a0),
                ruler: hex(0xc2c2c2),
                ruler_tick: hex(0x4a4a4a),
                measure_bg: Color32::from_rgba_unmultiplied(184, 184, 184, 235),
                button: hex(0xcacaca),
                button_border: hex(0x7a7a7a),
                flyout_active: hex(0x1473e6),
                ..base
            },
            Brightness::Light => Tokens {
                dark: false,
                app_bar: hex(0xe4e4e4),
                panel: hex(0xf0f0f0),
                panel_darker: hex(0xe2e2e2),
                tab_strip: hex(0xdddddd),
                input: hex(0xffffff),
                input_border: hex(0xb4b4b4),
                border: hex(0xcfcfcf),
                divider: hex(0xd6d6d6),
                text: hex(0x1f1f1f),
                text_strong: hex(0x000000),
                text_dim: hex(0x4b4b4b),
                text_disabled: hex(0x9a9a9a),
                icon: hex(0x303030),
                hover: hex(0xdddddd),
                tool_active: hex(0xcdcdcd),
                row_selected: hex(0xc8daf7),
                pasteboard: hex(0xdcdcdc),
                ruler: hex(0xeaeaea),
                ruler_tick: hex(0x5a5a5a),
                measure_bg: Color32::from_rgba_unmultiplied(240, 240, 240, 235),
                button: hex(0xe0e0e0),
                button_border: hex(0x9a9a9a),
                flyout_active: hex(0x1473e6),
                ..base
            },
        };
        classic.legacy_extras()
    }

    /// The tokens for `b` in `scheme`; Modern takes `accent`, Classic has its own blue. The Al
    /// Majlis looks keep their own colours in either scheme.
    pub fn for_scheme(scheme: Scheme, b: Brightness, accent: Accent) -> Self {
        if b.is_majlis() {
            return Self::for_brightness(b);
        }
        match scheme {
            Scheme::Modern => Self::for_brightness(b).with_accent(accent),
            Scheme::Classic => Self::classic(b),
        }
    }

    /// These tokens with `accent`: the accent, buttons filled with it, the active item in flyouts and
    /// selected rows (the panel tinted towards it).
    pub fn with_accent(self, accent: Accent) -> Self {
        if self.majlis {
            return self;
        }
        let hex = |s: u32| Color32::from_rgb((s >> 16) as u8, (s >> 8) as u8, s as u8);
        let (a, strong) = accent.colors(self.dark);
        let (a, strong) = (hex(a), hex(strong));
        let tint = |k: f32| {
            let mix = |p: u8, q: u8| (f32::from(p) + (f32::from(q) - f32::from(p)) * k).round() as u8;
            Color32::from_rgb(mix(self.panel.r(), a.r()), mix(self.panel.g(), a.g()), mix(self.panel.b(), a.b()))
        };
        Tokens {
            accent: a,
            accent_strong: strong,
            flyout_active: if self.dark { a } else { strong },
            row_selected: tint(if self.dark { 0.24 } else { 0.2 }),
            tool_active: tint(if self.dark { 0.3 } else { 0.24 }),
            ..self
        }
        .legacy_extras()
    }

    pub fn get(ctx: &egui::Context) -> Tokens {
        ctx.data(|d| d.get_temp::<Tokens>(egui::Id::NULL)).unwrap_or_else(|| Tokens::for_brightness(Brightness::default()))
    }
    pub fn cr(&self) -> CornerRadius {
        CornerRadius::same(self.radius)
    }
    /// A pill (the radius clamps to half the shape's height) in the Al Majlis look, else `legacy`.
    pub fn pill_or(&self, legacy: u8) -> CornerRadius {
        self.corner(self.pill_radius, legacy)
    }
    /// The shape token `majlis` (one of the `*_radius` fields) in the Al Majlis look, else the
    /// `legacy` corner the widget always had in the other looks.
    pub fn corner(&self, majlis: u8, legacy: u8) -> CornerRadius {
        CornerRadius::same(if self.majlis { majlis } else { legacy })
    }
}

/// The toolbar, the panel dock and the icon rail are cards: `body` drawn on the panel colour with
/// soft corners, `margin` of the window's frame colour around it.
pub fn card<R>(ui: &mut egui::Ui, t: &Tokens, margin: egui::Margin, body: impl FnOnce(&mut egui::Ui) -> R) -> R {
    egui::Frame::NONE
        .fill(t.app_bar)
        .inner_margin(margin)
        .show(ui, |ui| {
            let card = egui::Frame::NONE.fill(t.panel).corner_radius(t.card_radius).show(ui, |ui| {
                ui.set_min_size(ui.available_size());
                body(ui)
            });
            // The Al Majlis card's hairline, drawn over the card so its content keeps its size.
            if t.majlis {
                ui.painter().rect_stroke(card.response.rect, t.card_radius, Stroke::new(1.0, t.hairline), egui::StrokeKind::Inside);
            }
            card.inner
        })
        .inner
}

/// The corner radius of [`card`]s outside the Al Majlis look ([`Tokens::card_radius`]).
pub const CARD_RADIUS: u8 = 8;

/// Selected text in the text fields `ui` draws from here on is in the strong text colour. egui
/// paints it in the selection stroke's colour, the accent, which is hard to read on the selection's
/// accent tint (#1465). For fields that draw their own frame: egui's frame takes its focus ring from
/// the same stroke.
pub fn strong_selected_text(ui: &mut egui::Ui) {
    ui.visuals_mut().selection.stroke.color = Tokens::get(ui.ctx()).text_strong;
}

pub const FONT_UI: &str = "ui";
pub const FONT_UI_SEMIBOLD: &str = "ui-semibold";
pub const FONT_MONO: &str = "mono";

pub fn semibold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(FONT_UI_SEMIBOLD.into()))
}
pub fn mono(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(FONT_MONO.into()))
}

/// Installs the interface's fonts and binds the families the panels and dialogs draw in
/// ([`FONT_UI`], [`FONT_UI_SEMIBOLD`], [`FONT_MONO`]). egui 0.37 only bundles its own faces when
/// its `default_fonts` feature is on (this crate enables it); without any face a family lays out
/// tofu, and the `egui_kittest` harness alone panics on a missing glyph. Every context that shows
/// these families needs this once: [`crate::VectorcraftApp::ui`] calls it on its first pass, and
/// an embedder that draws the panels or dialogs without it calls it when it makes the context. It
/// replaces the context's font definitions: add other fonts after it, or keep these families in
/// definitions of one's own. Without the `interface-fonts` feature the families are egui's built-in
/// fonts (and craft-fonts' faces in a build with them), which lack some symbols the panels show
/// (→ ▾ ● ✓).
pub fn install_fonts(ctx: &egui::Context) {
    ctx.set_fonts(interface_fonts());
}

/// The definitions [`install_fonts`] installs. [`crate::VectorcraftApp::ui`] keeps them to tell
/// whether egui applied them at once or deferred them to the next pass (it must not take input
/// or draw with fallback metrics while they are pending).
pub fn interface_fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    let (ui, semi, mono) = interface_faces(&mut fonts);
    let stack = |fonts: &FontDefinitions, family: FontFamily| fonts.families.get(&family).cloned().unwrap_or_default();
    let (fallback, monospace) = (stack(&fonts, FontFamily::Proportional), stack(&fonts, FontFamily::Monospace));
    // Without its own monospace face, `mono` is egui's; with it, the face falls back to the text's.
    let mono_fallback = if mono.is_empty() { monospace } else { fallback.clone() };
    let family =
        |faces: Vec<&str>, fallback: &[String]| -> Vec<String> { faces.into_iter().map(str::to_owned).chain(fallback.iter().cloned()).collect() };
    fonts.families.insert(FontFamily::Proportional, family(ui.clone(), &fallback));
    fonts.families.insert(FontFamily::Name(FONT_UI_SEMIBOLD.into()), family(semi, &fallback));
    fonts.families.insert(FontFamily::Name(FONT_UI.into()), family(ui, &fallback));
    fonts.families.insert(FontFamily::Name(FONT_MONO.into()), family(mono, &mono_fallback));
    add_craft_fonts(&mut fonts);
    fonts
}

/// The interface's own faces, added to `fonts`: the names of the regular, semibold and monospace
/// faces that head their families, in order.
#[cfg(feature = "interface-fonts")]
fn interface_faces(fonts: &mut FontDefinitions) -> (Vec<&'static str>, Vec<&'static str>, Vec<&'static str>) {
    let add = |f: &mut FontDefinitions, name: &str, bytes: &'static [u8]| {
        f.font_data.insert(name.to_owned(), Arc::new(FontData::from_static(bytes)));
    };
    add(fonts, "SourceSans3", include_bytes!("../../../assets/fonts/SourceSans3-Regular.ttf"));
    // Inter is drawn a little smaller than its size: its letters are wider than Source Sans's, and
    // the panels are laid out for those.
    let inter =
        |bytes: &'static [u8]| Arc::new(FontData::from_static(bytes).tweak(egui::epaint::text::FontTweak { scale: 0.92, ..Default::default() }));
    fonts.font_data.insert("Inter".to_owned(), inter(include_bytes!("../../../assets/fonts/Inter-Regular.ttf")));
    fonts.font_data.insert("Inter-SemiBold".to_owned(), inter(include_bytes!("../../../assets/fonts/Inter-SemiBold.ttf")));
    add(fonts, "SourceSans3-Semibold", include_bytes!("../../../assets/fonts/SourceSans3-Semibold.ttf"));
    add(fonts, "JetBrainsMono", include_bytes!("../../../assets/fonts/JetBrainsMono-Regular.ttf"));
    (vec!["Inter", "SourceSans3"], vec!["Inter-SemiBold", "SourceSans3-Semibold"], vec!["JetBrainsMono"])
}

/// Without the `interface-fonts` feature the interface has no faces of its own: every family is egui's.
#[cfg(not(feature = "interface-fonts"))]
fn interface_faces(_fonts: &mut FontDefinitions) -> (Vec<&'static str>, Vec<&'static str>, Vec<&'static str>) {
    (Vec::new(), Vec::new(), Vec::new())
}

/// The egui name of a craft-fonts face.
pub(crate) fn craft_font_name(f: &vectorcraft_text::CraftFont) -> String {
    format!("craft-fonts {} {}", f.family, f.style)
}

/// The Japanese and Georgian craft-fonts faces (when built with `CRAFT_FONTS_DIR`) as fallbacks at
/// the end of every family, after the app's own fonts and before the installed fonts `ui_fonts`
/// adds: BIZ UDPGothic first (bold first in the semibold family), then the Mincho faces, then Noto
/// Sans Georgian.
fn add_craft_fonts(fonts: &mut FontDefinitions) {
    for f in vectorcraft_text::CRAFT_FONTS.iter().filter(|f| f.is_japanese() || f.is_georgian()) {
        fonts.font_data.insert(craft_font_name(f), Arc::new(FontData::from_static(f.bytes)));
    }
    for (family, stack) in fonts.families.iter_mut() {
        let bold = *family == FontFamily::Name(FONT_UI_SEMIBOLD.into());
        stack.extend(vectorcraft_text::craft_fonts::japanese_ui_fonts(bold).into_iter().map(craft_font_name));
        stack.extend(vectorcraft_text::craft_fonts::georgian_ui_fonts().into_iter().map(craft_font_name));
    }
}

/// Apply tokens to egui's global style.
pub fn apply(ctx: &egui::Context, b: Brightness) {
    apply_with(ctx, b, Accent::default(), Scheme::default());
}

/// [`apply`] with an accent colour and colour scheme.
pub fn apply_with(ctx: &egui::Context, b: Brightness, accent: Accent, scheme: Scheme) {
    let t = Tokens::for_scheme(scheme, b, accent);
    ctx.data_mut(|d| d.insert_temp(egui::Id::NULL, t));
    let mut v = if t.dark { Visuals::dark() } else { Visuals::light() };
    v.panel_fill = t.panel;
    v.window_fill = t.panel;
    v.extreme_bg_color = t.input;
    v.faint_bg_color = t.panel_darker;
    v.window_stroke = Stroke::new(1.0, if t.majlis { t.hairline } else { t.border });
    v.window_corner_radius = CornerRadius::same(t.window_radius);
    v.menu_corner_radius = CornerRadius::same(t.menu_radius);
    v.selection.bg_fill = if t.majlis { t.accent_soft } else { t.accent.gamma_multiply(0.55) };
    // Al Majlis: a 1.5 pt accent focus outline on fields.
    v.selection.stroke = Stroke::new(if t.majlis { 1.5 } else { 1.0 }, t.accent);
    v.hyperlink_color = if t.majlis { t.accent_text } else { t.accent };
    v.override_text_color = Some(t.text);
    // Al Majlis: soft shadows, deeper in the dark look.
    let (popup_alpha, window_alpha) = match (t.majlis, t.dark) {
        (true, true) => (140, 160),
        (true, false) => (28, 36),
        (false, true) => (120, 140),
        (false, false) => (60, 70),
    };
    v.popup_shadow = egui::epaint::Shadow { offset: [0, 4], blur: 14, spread: 0, color: Color32::from_black_alpha(popup_alpha) };
    v.window_shadow = egui::epaint::Shadow { offset: [0, 8], blur: 28, spread: 0, color: Color32::from_black_alpha(window_alpha) };
    let w = &mut v.widgets;
    if t.majlis {
        // Secondary buttons: raised fill and a 1 px hairline, pressed under the pointer. Scroll
        // bars and slider rails take the pressed colour (`bg_fill`), a bar under the pointer the
        // muted text.
        craft_ui::theme::WidgetPalette { noninteractive: t.panel, inactive: t.raised, hovered: t.pressed, active: t.pressed, open: t.pressed }
            .apply_to(w, t.cr(), Stroke::new(1.0, t.text));
        w.inactive.bg_fill = t.pressed;
        w.hovered.bg_fill = t.text_muted;
        w.noninteractive.bg_stroke = Stroke::new(1.0, t.hairline);
        w.inactive.bg_stroke = Stroke::new(1.0, t.hairline);
        w.hovered.bg_stroke = Stroke::new(1.0, t.hairline);
        w.active.bg_stroke = Stroke::new(1.0, t.accent);
        w.open.bg_stroke = Stroke::new(1.0, t.hairline);
    } else {
        craft_ui::theme::WidgetPalette { noninteractive: t.panel, inactive: t.button, hovered: t.hover, active: t.tool_active, open: t.hover }
            .apply_to(w, t.cr(), Stroke::new(1.0, t.text));
        w.noninteractive.bg_stroke = Stroke::new(1.0, t.divider);
        w.inactive.bg_stroke = Stroke::NONE;
        w.hovered.bg_stroke = Stroke::new(1.0, t.input_border);
        w.active.bg_stroke = Stroke::new(1.0, t.accent);
    }
    ctx.set_visuals(v);
    ctx.global_style_mut(|s| {
        s.spacing.item_spacing = egui::vec2(6.0, 5.0);
        s.spacing.button_padding = egui::vec2(6.0, 2.0);
        s.spacing.interact_size = egui::vec2(24.0, 24.0);
        s.spacing.menu_margin = egui::Margin::same(6);
        s.spacing.slider_width = 120.0;
        s.text_styles = [
            (TextStyle::Small, FontId::proportional(11.0)),
            (TextStyle::Body, FontId::proportional(13.0)),
            (TextStyle::Button, FontId::proportional(13.0)),
            (TextStyle::Heading, FontId::new(15.0, FontFamily::Name(FONT_UI_SEMIBOLD.into()))),
            (TextStyle::Monospace, FontId::new(11.5, FontFamily::Name(FONT_MONO.into()))),
        ]
        .into();
        s.animation_time = 0.08;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brightness_roundtrip() {
        for b in Brightness::ALL {
            assert_eq!(Brightness::parse(b.id()), Some(b));
            assert_eq!(Brightness::parse(b.label()), Some(b));
        }
        assert!(!Tokens::for_brightness(Brightness::Light).dark);
    }

    /// #1662: a slider's rail (`button_border`, see `widgets::slider`) stands out from the panel
    /// in every brightness.
    #[test]
    fn slider_rails_show_on_every_panel() {
        for b in Brightness::ALL {
            for scheme in Scheme::ALL {
                let t = Tokens::for_scheme(scheme, b, Accent::default());
                let gap = (i32::from(t.rail.r()) - i32::from(t.panel.r())).abs();
                // The Al Majlis track is its pressed colour (MAJLIS_LOOK.md), 23 levels off the panel.
                let least = if b.is_majlis() { 20 } else { 24 };
                assert!(gap >= least, "{scheme:?} {b:?}: rail {:?} on panel {:?}", t.rail, t.panel);
            }
        }
    }

    /// Classic keeps the Modern corners and the cards' frame, and has its own blue accent
    /// whatever accent is chosen.
    #[test]
    fn classic_scheme_keeps_the_corners_and_its_blue() {
        for b in Brightness::ALL.into_iter().filter(|b| !b.is_majlis()) {
            let (modern, classic) = (Tokens::for_brightness(b), Tokens::for_scheme(Scheme::Classic, b, Accent::Pink));
            assert_eq!(classic.radius, modern.radius);
            assert_eq!(classic.dark, modern.dark, "{b:?}");
            assert_ne!(classic.app_bar, classic.panel, "{b:?}: the cards need a frame");
            assert_eq!(classic.accent, Color32::from_rgb(0x37, 0x8e, 0xf0));
            assert_eq!(Scheme::parse(Scheme::Classic.id()), Some(Scheme::Classic));
        }
    }

    /// The Al Majlis looks keep their exact colours whatever accent or scheme is chosen, and the
    /// light one is the default.
    #[test]
    fn majlis_looks_keep_their_colours() {
        let hex = |s: u32| Color32::from_rgb((s >> 16) as u8, (s >> 8) as u8, s as u8);
        assert_eq!(Brightness::default(), Brightness::MajlisLight);
        for (b, canvas, surface) in [(Brightness::MajlisLight, 0xF2F1EE, 0xFFFFFF), (Brightness::MajlisDark, 0x0D0D0F, 0x17171A)] {
            for scheme in Scheme::ALL {
                let t = Tokens::for_scheme(scheme, b, Accent::Pink);
                assert!(t.majlis);
                assert_eq!(t.dark, b == Brightness::MajlisDark);
                assert_eq!(t.app_bar, hex(canvas));
                assert_eq!(t.panel, hex(surface));
                assert_eq!(t.accent, hex(0x2563EB));
                assert_eq!((t.card_radius, t.window_radius, t.menu_radius, t.tool_radius), (12, 16, 12, 8));
            }
            assert_eq!(Brightness::parse(b.id()), Some(b));
        }
    }
}
