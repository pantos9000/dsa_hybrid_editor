#[derive(Debug, Default, Copy, Clone)]
pub struct InvalidGradientValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gradient {
    value: i8,
}

impl Default for Gradient {
    fn default() -> Self {
        Self { value: 100 }
    }
}

impl TryFrom<i8> for Gradient {
    type Error = InvalidGradientValue;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            ..-100 => Err(InvalidGradientValue),
            101.. => Err(InvalidGradientValue),
            value => Ok(Self { value }),
        }
    }
}
