//! Payload Models

#[derive(Debug, Clone, PartialEq)]
pub struct Insert {
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub health: String,
    pub metadata: Option<MetaData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetaData {
    pub env: Option<String>,
    pub version: Option<String>,
    pub region: Option<String>,
    pub team: Option<String>,
    pub tags: Option<Vec<String>>,
}
