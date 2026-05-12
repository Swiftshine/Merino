use std::fmt::Display;


#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
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

impl<const N: usize> Display for LimitedString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub type String16 = LimitedString<16>;
pub type String32 = LimitedString<32>;
pub type String64 = LimitedString<64>;
