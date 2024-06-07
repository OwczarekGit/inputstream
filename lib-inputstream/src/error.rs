#[derive(Debug, Clone)]
pub enum MessageDeserialize {
    CouldNotParseStruct,
    CouldNotParseLen,
}
