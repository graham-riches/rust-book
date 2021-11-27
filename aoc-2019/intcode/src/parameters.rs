use num_derive::FromPrimitive;    

#[derive(FromPrimitive, Debug, PartialEq)]
pub enum ParameterMode {
    Position = 0,
    Immediate = 1,
}

impl Default for ParameterMode {
    fn default() -> Self {
        Self::Position
    }
}

