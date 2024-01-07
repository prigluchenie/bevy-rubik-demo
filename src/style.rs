use bevy::render::color::Color;

pub const COLOR_LEFT : Color = Color::BLUE;
pub const COLOR_RIGHT : Color = Color::GREEN;
pub const COLOR_BOTTOM : Color = Color::YELLOW;
pub const COLOR_TOP : Color = Color::WHITE;
pub const COLOR_BACK : Color = Color::PURPLE;
pub const COLOR_FRONT : Color = Color::RED;

pub const COLORS: [&Color; 6] = [&COLOR_LEFT, &COLOR_RIGHT, &COLOR_BOTTOM, &COLOR_TOP, &COLOR_BACK, &COLOR_FRONT];

pub const COLOR_BEVEL : Color = Color::BLACK;

pub const BEVEL_FRACTION: f32 = 0.2;
