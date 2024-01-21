use ratatui::style::Color;

pub(super) struct Palette {
    pub c50: Color,
    pub c100: Color,
    pub c200: Color,
    pub c300: Color,
    pub c400: Color,
    pub c500: Color,
    pub c600: Color,
    pub c700: Color,
    pub c800: Color,
    pub c900: Color,
    pub c950: Color,
}

pub(super) const SLATE: Palette = Palette {
    c50: from_u32(0xf8fafc),
    c100: from_u32(0xf1f5f9),
    c200: from_u32(0xe2e8f0),
    c300: from_u32(0xcbd5e1),
    c400: from_u32(0x94a3b8),
    c500: from_u32(0x64748b),
    c600: from_u32(0x475569),
    c700: from_u32(0x334155),
    c800: from_u32(0x1e293b),
    c900: from_u32(0x0f172a),
    c950: from_u32(0x020617),
};

pub(super) const PURPLE: Palette = Palette {
    c50: from_u32(0xfaf5ff),
    c100: from_u32(0xf3e8ff),
    c200: from_u32(0xe9d5ff),
    c300: from_u32(0xd8b4fe),
    c400: from_u32(0xc084fc),
    c500: from_u32(0xa855f7),
    c600: from_u32(0x9333ea),
    c700: from_u32(0x7e22ce),
    c800: from_u32(0x6b21a8),
    c900: from_u32(0x581c87),
    c950: from_u32(0x3b0764),
};

const fn from_u32(u: u32) -> Color {
    let r = (u >> 16) as u8;
    let g = (u >> 8) as u8;
    let b = u as u8;
    Color::Rgb(r, g, b)
}
