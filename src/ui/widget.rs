use egui_multiwin::egui::{Align2, Color32, FontId, Pos2, pos2, Rect, TextureHandle, Ui};
pub enum Widget {
    Text(TextWidget),
    Image(ImageWidget)
}

impl Widget {
    pub fn draw(&mut self, ui: &mut Ui, scale: f32) {
        match self {
            Widget::Text(tw) => tw.draw(ui, scale),
            Widget::Image(iw) => iw.draw(ui, scale)
        }
    }
    pub fn new(text: String, alignment: Align2, rect: Rect, font: FontId, padding: Padding, color: Color32) -> Self {
        Widget::Text(TextWidget {
            text,
            alignment,
            rect,
            font,
            padding,
            color,
        })
    }
}

pub struct ImageWidget {
    pub name: String,
    pub rect: Rect,
    texture_handle: Option<TextureHandle>,
}

impl ImageWidget {
    pub fn new(name: String, texture_handle: Option<TextureHandle>, rect: Rect) -> Self {
        Self {
            name,
            rect,
            texture_handle,
        }
    }

    pub fn draw(&self, ui: &mut Ui, _scale: f32) {
        if let Some(handle) = &self.texture_handle {
            ui.painter().image(handle.id(), self.rect, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
        }
    }
}

pub struct TextWidget {
    pub text: String,
    pub alignment: Align2,
    pub rect: Rect,
    pub font: FontId,
    pub padding: Padding,
    pub color: Color32,
}

impl TextWidget {
    pub fn draw(&self, ui: &mut Ui, scale: f32) {
        let pos = self.rect.calc_pos(self.alignment, self.padding, scale);

        let font = FontId {
            size: (self.font.size) * scale,
            ..self.font.clone()
        };

        ui.painter()
            .text(pos, self.alignment, &self.text, font.clone(), self.color);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Padding {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32
}

impl Padding {
    pub fn x(amount: f32) -> Self {
        Self {
            top: 0.0,
            bottom: 0.0,
            left: amount,
            right: amount
        }
    }
    pub fn y(amount: f32) -> Self {
        Self {
            top: amount,
            bottom: amount,
            left: 0.0,
            right: 0.0
        }
    }
    pub fn all(amount: f32) -> Self {
        Self {
            top: amount,
            bottom: amount,
            left: amount,
            right: amount
        }
    }
    pub fn none() -> Self {
        Self {
            top: 0.0,
            bottom: 0.0,
            left: 0.0,
            right: 0.0
        }
    }

    pub fn left(amount: f32) -> Self {
        Self {
            top: 0.0,
            bottom: 0.0,
            left: amount,
            right: 0.0
        }
    }

    pub fn right(amount: f32) -> Self {
        Self {
            top: 0.0,
            bottom: 0.0,
            left: 0.0,
            right: amount
        }
    }

    pub fn top(amount: f32) -> Self {
        Self {
            top: amount,
            bottom: 0.0,
            left: 0.0,
            right: 0.0
        }
    }

    pub fn bottom(amount: f32) -> Self {
        Self {
            top: 0.0,
            bottom: amount,
            left: 0.0,
            right: 0.0
        }
    }
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self{
            top, bottom, left, right
        }
    }
}

pub trait CalculatePosition {
    fn calc_pos(&self, alignment: Align2, padding: Padding, scale: f32) -> Pos2;
}

impl CalculatePosition for Rect {
    fn calc_pos(&self, alignment: Align2, padding: Padding, scale: f32) -> Pos2 {
        match alignment {
            Align2::LEFT_CENTER => Pos2 {
                x: self.min.x + (padding.left * scale),
                y: (self.min.y + self.max.y) / 2.0,
            },
            Align2::CENTER_TOP => Pos2 {
                x: (self.min.x + self.max.x) / 2.0,
                y: self.min.y + (padding.top * scale),
            },
            Align2::CENTER_CENTER => Pos2 {
                x: (self.min.x + self.max.x) / 2.0,
                y: (self.min.y + self.max.y) / 2.0,
            },
            Align2:: LEFT_BOTTOM => Pos2 {
                x: self.min.x + (padding.left * scale),
                y: self.max.y - (padding.bottom * scale),
            },
            Align2:: LEFT_TOP => Pos2 {
                x: self.min.x + (padding.left * scale),
                y: self.min.y + (padding.top * scale),
            },
            _ => todo!(),
        }
    }
}