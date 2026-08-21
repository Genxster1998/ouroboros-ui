//! Badge atom — a static pill label with shadcn variants (+ domain status).
//!
//! Variant → [`BadgeTokens`]; pill + centered text galley (token fonts/colors), optional
//! status dot. Atom paints its content (allowed).

use crate::theme::typography;
use crate::tokens::component::BadgeTokens;
use crate::tokens::core::{self, Size};
use crate::Theme;
use egui::{
    pos2, text::LayoutJob, text::TextFormat, vec2, CornerRadius, Response, Sense, Stroke,
    StrokeKind, Ui,
};

/// Badge variant — shadcn 6 + domain status.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
    Success,
    Warning,
    Info,
}

/// A static badge. Builder; `show` returns the [`Response`].
pub struct Badge {
    text: String,
    variant: BadgeVariant,
    dot: bool,
    size: Size,
    is_pill: bool,
    custom_radius: Option<f32>,
}

impl Badge {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            variant: BadgeVariant::default(),
            dot: false,
            size: Size::default(),
            is_pill: false,
            custom_radius: None,
        }
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
    pub fn sm(self) -> Self {
        self.size(Size::Sm)
    }
    pub fn lg(self) -> Self {
        self.size(Size::Lg)
    }
    pub fn secondary(self) -> Self {
        self.variant(BadgeVariant::Secondary)
    }
    pub fn destructive(self) -> Self {
        self.variant(BadgeVariant::Destructive)
    }
    pub fn outline(self) -> Self {
        self.variant(BadgeVariant::Outline)
    }
    pub fn ghost(self) -> Self {
        self.variant(BadgeVariant::Ghost)
    }
    pub fn link(self) -> Self {
        self.variant(BadgeVariant::Link)
    }
    pub fn success(self) -> Self {
        self.variant(BadgeVariant::Success)
    }
    pub fn warning(self) -> Self {
        self.variant(BadgeVariant::Warning)
    }
    pub fn info(self) -> Self {
        self.variant(BadgeVariant::Info)
    }
    /// Force pill shape (fully rounded ends).
    pub fn pill(mut self) -> Self {
        self.is_pill = true;
        self
    }
    /// Custom corner radius.
    pub fn radius(mut self, radius: f32) -> Self {
        self.custom_radius = Some(radius);
        self
    }
    /// Show a colored leading dot (status badge).
    pub fn dot(mut self) -> Self {
        self.dot = true;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui);
        let bt = match self.variant {
            BadgeVariant::Default => BadgeTokens::default_(&theme),
            BadgeVariant::Secondary => BadgeTokens::secondary(&theme),
            BadgeVariant::Destructive => BadgeTokens::destructive(&theme),
            BadgeVariant::Outline => BadgeTokens::outline(&theme),
            BadgeVariant::Ghost => BadgeTokens::ghost(&theme),
            BadgeVariant::Link => BadgeTokens::link(&theme),
            BadgeVariant::Success => BadgeTokens::success(&theme),
            BadgeVariant::Warning => BadgeTokens::warning(&theme),
            BadgeVariant::Info => BadgeTokens::info(&theme),
        };

        // Padding + text style scale with Size; control height aligns with buttons in the same row.
        let text_style = match self.size {
            Size::Sm => typography::caption(),
            Size::Md => typography::caption(),
            Size::Lg => typography::label(),
        };
        let pad_x = self.size.pad_x();
        let gap = core::SPACE_1;
        let dot_d = core::SPACE_2;

        let mut job = LayoutJob::default();
        job.append(
            &self.text,
            0.0,
            TextFormat {
                font_id: text_style.font_id(),
                color: bt.foreground,
                extra_letter_spacing: text_style.tracking,
                underline: if bt.underline {
                    Stroke::new(core::BORDER_THIN, bt.foreground)
                } else {
                    Stroke::NONE
                },
                ..Default::default()
            },
        );
        let galley = ui.painter().layout_job(job);
        let text_size = galley.size();

        let content_w = if self.dot { dot_d + gap } else { 0.0 } + text_size.x;
        let height = self.size.height();
        let size = vec2(content_w + 2.0 * pad_x, height);
        let (rect, response) = ui.allocate_exact_size(size, Sense::hover());

        let corner_radius = if let Some(r) = self.custom_radius {
            CornerRadius::same(r as u8)
        } else if self.is_pill {
            CornerRadius::same((rect.height() / 2.0) as u8)
        } else {
            CornerRadius::same(core::RADIUS_SM as u8)
        };

        let painter = ui.painter().clone();
        if bt.fill.a() > 0 {
            painter.rect_filled(rect, corner_radius, bt.fill);
        }
        if bt.border.a() > 0 {
            painter.rect_stroke(
                rect,
                corner_radius,
                Stroke::new(core::BORDER_THIN, bt.border),
                StrokeKind::Inside,
            );
        }

        let mut x = rect.left() + pad_x;
        if self.dot {
            painter.circle_filled(
                pos2(x + dot_d / 2.0, rect.center().y),
                dot_d / 2.0,
                bt.foreground,
            );
            x += dot_d + gap;
        }
        painter.galley(
            pos2(x, rect.center().y - text_size.y / 2.0),
            galley,
            bt.foreground,
        );

        response
    }
}
