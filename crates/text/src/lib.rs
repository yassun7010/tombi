mod features;
mod position;
mod range;
mod size;
mod traits;

pub use crate::{range::TextRange, size::TextSize, traits::TextLen};

#[cfg(target_pointer_width = "16")]
compile_error!("text-size assumes usize >= u32 and does not work on 16-bit targets");
