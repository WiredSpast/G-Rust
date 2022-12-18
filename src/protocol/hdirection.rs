#[derive(Debug, Clone, PartialEq)]
pub enum HDirection {
    ToClient,
    ToServer,
    None
}

impl Default for HDirection {
    fn default() -> Self {
        HDirection::None
    }
}