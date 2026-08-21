//! Typography — font registration and composite type styles.
//!
//! Uses system default proportional and monospace typography plus Phosphor **Light** icons.

use crate::tokens::core::{self, Size};
use egui::{FontDefinitions, FontFamily, FontId};

/// Font weight placeholder for system default type styles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Weight {
    Light,
    Regular,
    Medium,
    SemiBold,
    Bold,
}

fn sans(_weight: Weight) -> FontFamily {
    FontFamily::Proportional
}

fn mono(_bold: bool) -> FontFamily {
    FontFamily::Monospace
}

/// Register system default fonts + Phosphor icons into [`FontDefinitions`].
pub fn register(fonts: &mut FontDefinitions) {
    // Phosphor Light — registers the "phosphor" face and appends it to Proportional only.
    egui_phosphor::add_to_fonts(fonts, egui_phosphor::Variant::Light);

    // Append Phosphor as fallback to Monospace so icons resolve inline everywhere
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("phosphor".to_owned());
}

/// A composite typography token — family (incl. weight), size, resolved line-height
/// (px) and letter tracking. Build an [`egui::FontId`] with [`TypeStyle::font_id`];
/// the line-height/tracking are applied by the text atom when laying out a galley.
#[derive(Clone, Debug)]
pub struct TypeStyle {
    pub family: FontFamily,
    pub size: f32,
    pub line_height: f32,
    pub tracking: f32,
}

impl TypeStyle {
    /// The [`FontId`] (family + size) for this style.
    pub fn font_id(&self) -> FontId {
        FontId::new(self.size, self.family.clone())
    }
}

fn style(family: FontFamily, size: f32, leading: f32, tracking: f32) -> TypeStyle {
    TypeStyle {
        family,
        size,
        line_height: size * leading,
        tracking,
    }
}

// ── Named roles ──────────────────────────────────────────────────────────────

/// Largest title — 28.
pub fn display() -> TypeStyle {
    style(
        sans(Weight::Bold),
        core::TEXT_3XL,
        core::LEADING_TIGHT,
        core::TRACKING_NORMAL,
    )
}
/// H1 — 22.
pub fn h1() -> TypeStyle {
    style(
        sans(Weight::SemiBold),
        core::TEXT_2XL,
        core::LEADING_TIGHT,
        core::TRACKING_NORMAL,
    )
}
/// H2 — 18.
pub fn h2() -> TypeStyle {
    style(
        sans(Weight::SemiBold),
        core::TEXT_XL,
        core::LEADING_TIGHT,
        core::TRACKING_NORMAL,
    )
}
/// Section heading — 15.
pub fn heading() -> TypeStyle {
    style(
        sans(Weight::SemiBold),
        core::TEXT_LG,
        core::LEADING_TIGHT,
        core::TRACKING_SM,
    )
}
/// Body — 13.5 (the default text weight).
pub fn body() -> TypeStyle {
    style(
        sans(Weight::Regular),
        core::TEXT_BASE,
        core::LEADING_NORMAL,
        core::TRACKING_MD,
    )
}
/// Emphasized body — 13.5.
pub fn body_strong() -> TypeStyle {
    style(
        sans(Weight::Medium),
        core::TEXT_BASE,
        core::LEADING_NORMAL,
        core::TRACKING_MD,
    )
}
/// Label — 12.5 (the default label weight).
pub fn label() -> TypeStyle {
    style(
        sans(Weight::Regular),
        core::TEXT_SM,
        core::LEADING_NORMAL,
        core::TRACKING_LG,
    )
}
/// Emphasized label — 12.5.
pub fn label_strong() -> TypeStyle {
    style(
        sans(Weight::Medium),
        core::TEXT_SM,
        core::LEADING_NORMAL,
        core::TRACKING_LG,
    )
}
/// Caption / small — 11.5.
pub fn caption() -> TypeStyle {
    style(
        sans(Weight::Regular),
        core::TEXT_XS,
        core::LEADING_NORMAL,
        core::TRACKING_WIDE,
    )
}
/// Inline code — Monospace 12.
pub fn code() -> TypeStyle {
    style(
        mono(false),
        core::TEXT_SM,
        core::LEADING_NORMAL,
        core::TRACKING_LG,
    )
}
/// Keyboard key — Monospace 11.
pub fn kbd() -> TypeStyle {
    style(
        mono(true),
        core::TEXT_XS,
        core::LEADING_NORMAL,
        core::TRACKING_WIDE,
    )
}

/// Font for an icon glyph at `size`. Phosphor glyphs resolve via the proportional stack.
pub fn icon_font(size: f32) -> FontId {
    FontId::new(size, FontFamily::Proportional)
}

impl Size {
    pub fn text_style(self) -> TypeStyle {
        match self {
            Size::Lg => body_strong(),
            Size::Sm | Size::Md => label(),
        }
    }
}
