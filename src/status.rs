#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Status {
    Hit,
    Miss,
    Unknown,
}