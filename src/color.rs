pub use palette::*;

pub type Color = palette::LinSrgba;

#[rustfmt::skip]
pub mod color{
    use super::Color;

    pub fn color_to_u8(color: &Color) -> [u8; 4] {
        let u8_vec: [u8; 4] = [
            (color.red * 256.0) as u8,
            (color.green * 256.0) as u8,
            (color.blue * 256.0) as u8,
            (color.alpha * 256.0) as u8,
        ];
        u8_vec
    }

    pub const BLACK: Color = Color { color: palette::rgb::Rgb { red: 0.0, green: 0.0, blue: 0.0, standard: std::marker::PhantomData }, alpha: 1.0  };
    pub const WHITE: Color = Color { color: palette::rgb::Rgb { red: 1.0, green: 1.0, blue: 1.0, standard: std::marker::PhantomData }, alpha: 1.0  };
    pub const TRANSPARENT: Color = Color{ color: palette::rgb::Rgb { red: 1.0, green: 1.0, blue: 1.0, standard: std::marker::PhantomData }, alpha: 0.0  };

    pub const RED: Color = Color { color: palette::rgb::Rgb { red: 1.0, green: 0.0, blue: 0.0, standard: std::marker::PhantomData }, alpha: 1.0  };
    pub const GREEN: Color = Color { color: palette::rgb::Rgb { red: 0.0, green: 1.0, blue: 0.0, standard: std::marker::PhantomData }, alpha: 1.0  };
    pub const BLUE: Color = Color { color: palette::rgb::Rgb { red: 0.0, green: 0.0, blue: 1.0, standard: std::marker::PhantomData }, alpha: 1.0  };
    pub const YELLOW: Color = Color { color: palette::rgb::Rgb { red: 1.0, green: 1.0, blue: 0.0, standard: std::marker::PhantomData }, alpha: 1.0  };
    pub const PURPLE: Color = Color { color: palette::rgb::Rgb { red: 1.0, green: 0.0, blue: 1.0, standard: std::marker::PhantomData }, alpha: 1.0  };
    pub const TEAL: Color = Color { color: palette::rgb::Rgb { red: 0.0, green: 1.0, blue: 1.0, standard: std::marker::PhantomData }, alpha: 1.0  };
    pub const PINK: Color = Color { color: palette::rgb::Rgb { red: 1.0, green: 0.0, blue: 0.672, standard: std::marker::PhantomData }, alpha: 1.0  };
    pub const ORANGE: Color = Color { color: palette::rgb::Rgb { red: 1.0, green: 0.348, blue: 0.0, standard: std::marker::PhantomData }, alpha: 1.0  };

    pub const DARK_RED: Color = Color { color: palette::rgb::Rgb { red: 0.37, green: 0.1, blue: 0.1, standard: std::marker::PhantomData }, alpha: 1.0  };
    pub const DARK_GREEN: Color = Color { color: palette::rgb::Rgb { red: 0.1, green: 0.37, blue: 0.1, standard: std::marker::PhantomData }, alpha: 1.0  };
    pub const DARK_BLUE: Color = Color { color: palette::rgb::Rgb { red: 0.1, green: 0.1, blue: 0.37, standard: std::marker::PhantomData }, alpha: 1.0  };
    pub const DARK_YELLOW: Color = Color { color: palette::rgb::Rgb { red: 0.37, green: 0.37, blue: 0.1, standard: std::marker::PhantomData }, alpha: 1.0  };
    pub const DARK_PURPLE: Color = Color { color: palette::rgb::Rgb { red: 0.37, green: 0.1, blue: 0.37, standard: std::marker::PhantomData }, alpha: 1.0  };
    pub const DARK_TEAL: Color = Color { color: palette::rgb::Rgb { red: 0.1, green: 0.37, blue: 0.37, standard: std::marker::PhantomData }, alpha: 1.0  };
    pub const DARK_PINK: Color = Color { color: palette::rgb::Rgb { red: 0.37, green: 0.1, blue:  0.24, standard: std::marker::PhantomData }, alpha: 1.0  };
    pub const DARK_ORANGE: Color = Color { color: palette::rgb::Rgb { red: 0.37, green: 0.18, blue: 0.1, standard: std::marker::PhantomData }, alpha: 1.0  };
}
