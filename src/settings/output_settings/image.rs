use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OutputImage {
    #[serde(default = "default_output_image_use_base64")]
    pub use_base64: bool,
}

fn default_output_image_use_base64() -> bool {
    OutputImage::default().use_base64
}

impl OutputImage {
    pub fn new(use_base64: Option<bool>) -> Self {
        Self {
            use_base64: match use_base64 {
                Some(b) => b,
                None => false,
            },
        }
    }
}

impl Default for OutputImage {
    fn default() -> Self {
        OutputImage::new(None)
    }
}
