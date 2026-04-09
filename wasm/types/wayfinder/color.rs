use crate::types::foundry::utils::JsColor;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const ALICE_BLUE: Color = Color { r: 240, g: 248, b: 255 };
    pub const ANTIQUE_WHITE: Color = Color { r: 250, g: 235, b: 215 };
    pub const AQUA: Color = Color { r: 0, g: 255, b: 255 };
    pub const AQUAMARINE: Color = Color { r: 127, g: 255, b: 212 };
    pub const AZURE: Color = Color { r: 240, g: 255, b: 255 };
    pub const BEIGE: Color = Color { r: 245, g: 245, b: 220 };
    pub const BISQUE: Color = Color { r: 255, g: 228, b: 196 };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const BLANCHED_ALMOND: Color = Color { r: 255, g: 235, b: 205 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };
    pub const BLUE_VIOLET: Color = Color { r: 138, g: 43, b: 226 };
    pub const BROWN: Color = Color { r: 165, g: 42, b: 42 };
    pub const BURLY_WOOD: Color = Color { r: 222, g: 184, b: 135 };
    pub const CADET_BLUE: Color = Color { r: 95, g: 158, b: 160 };
    pub const CHARTREUSE: Color = Color { r: 127, g: 255, b: 0 };
    pub const CHOCOLATE: Color = Color { r: 210, g: 105, b: 30 };
    pub const CORAL: Color = Color { r: 255, g: 127, b: 80 };
    pub const CORNFLOWER_BLUE: Color = Color { r: 100, g: 149, b: 237 };
    pub const CORNSILK: Color = Color { r: 255, g: 248, b: 220 };
    pub const CRIMSON: Color = Color { r: 220, g: 20, b: 60 };
    pub const CYAN: Color = Color { r: 0, g: 255, b: 255 };
    pub const DARK_BLUE: Color = Color { r: 0, g: 0, b: 139 };
    pub const DARK_CYAN: Color = Color { r: 0, g: 139, b: 139 };
    pub const DARK_GOLDEN_ROD: Color = Color { r: 184, g: 134, b: 11 };
    pub const DARK_GRAY: Color = Color { r: 169, g: 169, b: 169 };
    pub const DARK_GREY: Color = Color { r: 169, g: 169, b: 169 };
    pub const DARK_GREEN: Color = Color { r: 0, g: 100, b: 0 };
    pub const DARK_KHAKI: Color = Color { r: 189, g: 183, b: 107 };
    pub const DARK_MAGENTA: Color = Color { r: 139, g: 0, b: 139 };
    pub const DARK_OLIVE_GREEN: Color = Color { r: 85, g: 107, b: 47 };
    pub const DARK_ORANGE: Color = Color { r: 255, g: 140, b: 0 };
    pub const DARK_ORCHID: Color = Color { r: 153, g: 50, b: 204 };
    pub const DARK_RED: Color = Color { r: 139, g: 0, b: 0 };
    pub const DARK_SALMON: Color = Color { r: 233, g: 150, b: 122 };
    pub const DARK_SEA_GREEN: Color = Color { r: 143, g: 188, b: 143 };
    pub const DARK_SLATE_BLUE: Color = Color { r: 72, g: 61, b: 139 };
    pub const DARK_SLATE_GRAY: Color = Color { r: 47, g: 79, b: 79 };
    pub const DARK_SLATE_GREY: Color = Color { r: 47, g: 79, b: 79 };
    pub const DARK_TURQUOISE: Color = Color { r: 0, g: 206, b: 209 };
    pub const DARK_VIOLET: Color = Color { r: 148, g: 0, b: 211 };
    pub const DEEP_PINK: Color = Color { r: 255, g: 20, b: 147 };
    pub const DEEP_SKY_BLUE: Color = Color { r: 0, g: 191, b: 255 };
    pub const DIM_GRAY: Color = Color { r: 105, g: 105, b: 105 };
    pub const DIM_GREY: Color = Color { r: 105, g: 105, b: 105 };
    pub const DODGER_BLUE: Color = Color { r: 30, g: 144, b: 255 };
    pub const FIRE_BRICK: Color = Color { r: 178, g: 34, b: 34 };
    pub const FLORAL_WHITE: Color = Color { r: 255, g: 250, b: 240 };
    pub const FOREST_GREEN: Color = Color { r: 34, g: 139, b: 34 };
    pub const FUCHSIA: Color = Color { r: 255, g: 0, b: 255 };
    pub const GAINSBORO: Color = Color { r: 220, g: 220, b: 220 };
    pub const GHOST_WHITE: Color = Color { r: 248, g: 248, b: 255 };
    pub const GOLD: Color = Color { r: 255, g: 215, b: 0 };
    pub const GOLDEN_ROD: Color = Color { r: 218, g: 165, b: 32 };
    pub const GRAY: Color = Color { r: 128, g: 128, b: 128 };
    pub const GREY: Color = Color { r: 128, g: 128, b: 128 };
    pub const GREEN: Color = Color { r: 0, g: 128, b: 0 };
    pub const GREEN_YELLOW: Color = Color { r: 173, g: 255, b: 47 };
    pub const HONEY_DEW: Color = Color { r: 240, g: 255, b: 240 };
    pub const HOT_PINK: Color = Color { r: 255, g: 105, b: 180 };
    pub const INDIAN_RED: Color = Color { r: 205, g: 92, b: 92 };
    pub const INDIGO: Color = Color { r: 75, g: 0, b: 130 };
    pub const IVORY: Color = Color { r: 255, g: 255, b: 240 };
    pub const KHAKI: Color = Color { r: 240, g: 230, b: 140 };
    pub const LAVENDER: Color = Color { r: 230, g: 230, b: 250 };
    pub const LAVENDER_BLUSH: Color = Color { r: 255, g: 240, b: 245 };
    pub const LAWN_GREEN: Color = Color { r: 124, g: 252, b: 0 };
    pub const LEMON_CHIFFON: Color = Color { r: 255, g: 250, b: 205 };
    pub const LIGHT_BLUE: Color = Color { r: 173, g: 216, b: 230 };
    pub const LIGHT_CORAL: Color = Color { r: 240, g: 128, b: 128 };
    pub const LIGHT_CYAN: Color = Color { r: 224, g: 255, b: 255 };
    pub const LIGHT_GOLDEN_ROD_YELLOW: Color = Color { r: 250, g: 250, b: 210 };
    pub const LIGHT_GRAY: Color = Color { r: 211, g: 211, b: 211 };
    pub const LIGHT_GREY: Color = Color { r: 211, g: 211, b: 211 };
    pub const LIGHT_GREEN: Color = Color { r: 144, g: 238, b: 144 };
    pub const LIGHT_PINK: Color = Color { r: 255, g: 182, b: 193 };
    pub const LIGHT_SALMON: Color = Color { r: 255, g: 160, b: 122 };
    pub const LIGHT_SEA_GREEN: Color = Color { r: 32, g: 178, b: 170 };
    pub const LIGHT_SKY_BLUE: Color = Color { r: 135, g: 206, b: 250 };
    pub const LIGHT_SLATE_GRAY: Color = Color { r: 119, g: 136, b: 153 };
    pub const LIGHT_SLATE_GREY: Color = Color { r: 119, g: 136, b: 153 };
    pub const LIGHT_STEEL_BLUE: Color = Color { r: 176, g: 196, b: 222 };
    pub const LIGHT_YELLOW: Color = Color { r: 255, g: 255, b: 224 };
    pub const LIME: Color = Color { r: 0, g: 255, b: 0 };
    pub const LIME_GREEN: Color = Color { r: 50, g: 205, b: 50 };
    pub const LINEN: Color = Color { r: 250, g: 240, b: 230 };
    pub const MAGENTA: Color = Color { r: 255, g: 0, b: 255 };
    pub const MAROON: Color = Color { r: 128, g: 0, b: 0 };
    pub const MEDIUM_AQUA_MARINE: Color = Color { r: 102, g: 205, b: 170 };
    pub const MEDIUM_BLUE: Color = Color { r: 0, g: 0, b: 205 };
    pub const MEDIUM_ORCHID: Color = Color { r: 186, g: 85, b: 211 };
    pub const MEDIUM_PURPLE: Color = Color { r: 147, g: 112, b: 219 };
    pub const MEDIUM_SEA_GREEN: Color = Color { r: 60, g: 179, b: 113 };
    pub const MEDIUM_SLATE_BLUE: Color = Color { r: 123, g: 104, b: 238 };
    pub const MEDIUM_SPRING_GREEN: Color = Color { r: 0, g: 250, b: 154 };
    pub const MEDIUM_TURQUOISE: Color = Color { r: 72, g: 209, b: 204 };
    pub const MEDIUM_VIOLET_RED: Color = Color { r: 199, g: 21, b: 133 };
    pub const MIDNIGHT_BLUE: Color = Color { r: 25, g: 25, b: 112 };
    pub const MINT_CREAM: Color = Color { r: 245, g: 255, b: 250 };
    pub const MISTY_ROSE: Color = Color { r: 255, g: 228, b: 225 };
    pub const MOCCASIN: Color = Color { r: 255, g: 228, b: 181 };
    pub const NAVAJO_WHITE: Color = Color { r: 255, g: 222, b: 173 };
    pub const NAVY: Color = Color { r: 0, g: 0, b: 128 };
    pub const OLD_LACE: Color = Color { r: 253, g: 245, b: 230 };
    pub const OLIVE: Color = Color { r: 128, g: 128, b: 0 };
    pub const OLIVE_DRAB: Color = Color { r: 107, g: 142, b: 35 };
    pub const ORANGE: Color = Color { r: 255, g: 165, b: 0 };
    pub const ORANGE_RED: Color = Color { r: 255, g: 69, b: 0 };
    pub const ORCHID: Color = Color { r: 218, g: 112, b: 214 };
    pub const PALE_GOLDEN_ROD: Color = Color { r: 238, g: 232, b: 170 };
    pub const PALE_GREEN: Color = Color { r: 152, g: 251, b: 152 };
    pub const PALE_TURQUOISE: Color = Color { r: 175, g: 238, b: 238 };
    pub const PALE_VIOLET_RED: Color = Color { r: 219, g: 112, b: 147 };
    pub const PAPAYA_WHIP: Color = Color { r: 255, g: 239, b: 213 };
    pub const PEACH_PUFF: Color = Color { r: 255, g: 218, b: 185 };
    pub const PERU: Color = Color { r: 205, g: 133, b: 63 };
    pub const PINK: Color = Color { r: 255, g: 192, b: 203 };
    pub const PLUM: Color = Color { r: 221, g: 160, b: 221 };
    pub const POWDER_BLUE: Color = Color { r: 176, g: 224, b: 230 };
    pub const PURPLE: Color = Color { r: 128, g: 0, b: 128 };
    pub const REBECCA_PURPLE: Color = Color { r: 102, g: 51, b: 153 };
    pub const RED: Color = Color { r: 255, g: 0, b: 0 };
    pub const ROSY_BROWN: Color = Color { r: 188, g: 143, b: 143 };
    pub const ROYAL_BLUE: Color = Color { r: 65, g: 105, b: 225 };
    pub const SADDLE_BROWN: Color = Color { r: 139, g: 69, b: 19 };
    pub const SALMON: Color = Color { r: 250, g: 128, b: 114 };
    pub const SANDY_BROWN: Color = Color { r: 244, g: 164, b: 96 };
    pub const SEA_GREEN: Color = Color { r: 46, g: 139, b: 87 };
    pub const SEA_SHELL: Color = Color { r: 255, g: 245, b: 238 };
    pub const SIENNA: Color = Color { r: 160, g: 82, b: 45 };
    pub const SILVER: Color = Color { r: 192, g: 192, b: 192 };
    pub const SKY_BLUE: Color = Color { r: 135, g: 206, b: 235 };
    pub const SLATE_BLUE: Color = Color { r: 106, g: 90, b: 205 };
    pub const SLATE_GRAY: Color = Color { r: 112, g: 128, b: 144 };
    pub const SLATE_GREY: Color = Color { r: 112, g: 128, b: 144 };
    pub const SNOW: Color = Color { r: 255, g: 250, b: 250 };
    pub const SPRING_GREEN: Color = Color { r: 0, g: 255, b: 127 };
    pub const STEEL_BLUE: Color = Color { r: 70, g: 130, b: 180 };
    pub const TAN: Color = Color { r: 210, g: 180, b: 140 };
    pub const TEAL: Color = Color { r: 0, g: 128, b: 128 };
    pub const THISTLE: Color = Color { r: 216, g: 191, b: 216 };
    pub const TOMATO: Color = Color { r: 255, g: 99, b: 71 };
    pub const TURQUOISE: Color = Color { r: 64, g: 224, b: 208 };
    pub const VIOLET: Color = Color { r: 238, g: 130, b: 238 };
    pub const WHEAT: Color = Color { r: 245, g: 222, b: 179 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };
    pub const WHITE_SMOKE: Color = Color { r: 245, g: 245, b: 245 };
    pub const YELLOW: Color = Color { r: 255, g: 255, b: 0 };
    pub const YELLOW_GREEN: Color = Color { r: 154, g: 205, b: 50 };
}

impl From<[u8; 4]> for Color {
    fn from([b, g, r, _]: [u8; 4]) -> Self {
        Color { r, g, b }
    }
}

impl From<Color> for [u8; 4] {
    fn from(Color { r, g, b }: Color) -> Self {
        [b, g, r, 0]
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        value.to_le_bytes().into()
    }
}

impl From<Color> for u32 {
    fn from(value: Color) -> Self {
        u32::from_le_bytes(value.into())
    }
}

impl From<f64> for Color {
    fn from(value: f64) -> Self {
        Color::from(value as u32)
    }
}

impl From<Color> for f64 {
    fn from(value: Color) -> Self {
        u32::from(value) as f64
    }
}

impl From<JsColor> for Color {
    fn from(value: JsColor) -> Self {
        Color::from(value.value_of())
    }
}
