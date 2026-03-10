use anyhow::Result;
use hf_hub::api::sync::Api;
use std::path::PathBuf;

/// download the model using hf-hub
pub fn download_model() -> Result<(PathBuf, PathBuf)> {
    let api = Api::new()?;
    let repo = api.model("sentence-transformers/all-MiniLM-L6-v2".to_string());

    let model_path = repo.get("onnx/model.onnx")?;
    let tokenizer_path = repo.get("tokenizer.json")?;

    println!("Model downloaded to: {:?}", model_path);
    println!("Tokenizer downloaded to: {:?}", tokenizer_path);

    Ok((model_path, tokenizer_path))
}
