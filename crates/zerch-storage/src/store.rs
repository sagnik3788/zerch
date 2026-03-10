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

    // dump the vector into the bin , &[f32] for no copy of vector
    pub fn append_vector(&self, vector: &[f32]) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        // change this line
        let bytes = unsafe { vector.align_to::<u8>().1 };

        file.write_all(bytes)?;
        file.flush()?;

        Ok(())
    }
}
