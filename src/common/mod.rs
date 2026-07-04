use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutMessage {
    pub process_name: String,
    pub layout_id: u32,
    pub thread_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    SetLayout { process_name: String, layout_id: u32 },
    GetLayout { process_name: String },
}