use anyhow::Result;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

// path to store the .bin
pub struct VectorStore {
    pub path: PathBuf,
}

impl VectorStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    // dump the vector into the bin , &[f32] for no copy of vector, along with the text
    pub fn append_vector(&self, vector: &[f32], text: &str) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        // Write the length of the vector
        let vector_len = vector.len() as u32;
        file.write_all(&vector_len.to_le_bytes())?;

        // Write the vector data itself
        let bytes = unsafe {
            std::slice::from_raw_parts(
                vector.as_ptr() as *const u8,
                vector.len() * std::mem::size_of::<f32>(),
            )
        };
        file.write_all(bytes)?;

        // Write the length of the text
        let text_len = text.len() as u32;
        file.write_all(&text_len.to_le_bytes())?;

        // Write the text bytes
        file.write_all(text.as_bytes())?;

        file.flush()?;

        Ok(())
    }
}
