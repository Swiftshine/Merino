use std::fmt::Display;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub trait Vec2Like {
    // fn x(&self) -> f32;
    // fn y(&self) -> f32;
    fn x_mut(&mut self) -> &mut f32;
    fn y_mut(&mut self) -> &mut f32;
}

impl Vec2Like for Vec2f {
    // fn x(&self) -> f32 {
    //     self.x
    // }

    // fn y(&self) -> f32 {
    //     self.y
    // }

    fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }

    fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
}

impl Vec2Like for Vec3f {
    // fn x(&self) -> f32 {
    //     self.x
    // }

    // fn y(&self) -> f32 {
    //     self.y
    // }

    fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
}

#[derive(Debug, Clone)]
pub struct Params<const N: usize> {
    pub int_values: [i32; N],
    pub float_values: [f32; N],
    pub string_values: [String64; N],
}

impl<const N: usize> Default for Params<N> {
    fn default() -> Self {
        Self {
            int_values: [0; N],
            float_values: [0.0; N],
            string_values: std::array::from_fn(|_| Default::default()),
        }
    }
}

pub enum AnyParams<'a> {
    Params3(&'a Params<3>),
    Params5(&'a Params<5>),
}

impl<'a> From<&'a Params<3>> for AnyParams<'a> {
    fn from(p: &'a Params<3>) -> Self {
        Self::Params3(p)
    }
}

impl<'a> From<&'a Params<5>> for AnyParams<'a> {
    fn from(p: &'a Params<5>) -> Self {
        Self::Params5(p)
    }
}

impl<'a> AnyParams<'a> {
    // pub fn len(&self) -> usize {
    //     match self {
    //         Self::Params3(_) => 3,
    //         Self::Params5(_) => 5
    //     }
    // }

    pub fn int_params(&self) -> &[i32] {
        match self {
            Self::Params3(p) => &p.int_values,
            Self::Params5(p) => &p.int_values,
        }
    }

    pub fn float_params(&self) -> &[f32] {
        match self {
            Self::Params3(p) => &p.float_values,
            Self::Params5(p) => &p.float_values,
        }
    }

    pub fn string_params(&self) -> &[String64] {
        match self {
            Self::Params3(p) => &p.string_values,
            Self::Params5(p) => &p.string_values,
        }
    }
}

/// A string with a char limit
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LimitedString<const N: usize>(pub String);

impl<const N: usize> LimitedString<N> {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<const N: usize> From<String> for LimitedString<N> {
    fn from(string: String) -> Self {
        Self(string)
    }
}

impl<const N: usize> From<&str> for LimitedString<N> {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl<const N: usize> Display for LimitedString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub type String16 = LimitedString<16>;
pub type String32 = LimitedString<32>;
pub type String64 = LimitedString<64>;

// pub enum AnyLimitedString {
//     String16(String16),
//     String32(String32),
//     String64(String64),
// }
