use crate::downloader::download_model;
use anyhow::Result;
use ort::execution_providers::CUDAExecutionProvider;
use ort::session::Session;
use tokenizers::Tokenizer;

// LocalEmbedder load the model once
pub struct LocalEmbedder {
    pub(crate) tokenizer: Tokenizer,
    pub(crate) session: Session,
}

impl LocalEmbedder {
    // load the model
    pub fn load() -> Result<Self> {
        let (model_path, tokenizer_path) = download_model()?;
        let tokenizer = Tokenizer::from_file(tokenizer_path).map_err(anyhow::Error::msg)?;

        // in ort=2.0, it can directly check ur gpu/cpu
        // TODO: Remove the cuda enabled ort version, and make it optional
        let session = Session::builder()
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .with_execution_providers([CUDAExecutionProvider::default().build()])
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .commit_from_file(model_path)?;

        Ok(LocalEmbedder { tokenizer, session })
    }

    // fn to generate vector (refactor this mess)
    pub fn embed(&mut self, text: &str) -> Result<Vec<f32>> {
        // Tokenization
        let encoding = self
            .tokenizer
            .encode(text, true)
            .map_err(anyhow::Error::msg)?;
        let mut ids = encoding
            .get_ids()
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>();
        let mut mask = encoding
            .get_attention_mask()
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>();
        let mut type_ids = encoding
            .get_type_ids()
            .iter()
            .map(|&x| x as i64)
            .collect::<Vec<_>>();

        // As per the model's token limit, truncate to 512 tokens
        // TODO: Find a better way to handle large logs strings
        ids.truncate(512);
        mask.truncate(512);
        type_ids.truncate(512);
        let n_tokens = ids.len();

        // Prepare inputs
        let input_ids =
            ort::value::Value::from_array(ndarray::Array2::from_shape_vec((1, n_tokens), ids)?)?;
        let attention_mask =
            ort::value::Value::from_array(ndarray::Array2::from_shape_vec((1, n_tokens), mask)?)?;
        let token_type_ids = ort::value::Value::from_array(ndarray::Array2::from_shape_vec(
            (1, n_tokens),
            type_ids,
        )?)?;

        let session_inputs = ort::inputs![
            "input_ids" => &input_ids,
            "attention_mask" => &attention_mask,
            "token_type_ids" => &token_type_ids,
        ];

        // Run Inference
        let outputs = self.session.run(session_inputs)?;
        let view = outputs["last_hidden_state"].try_extract_array::<f32>()?; // Shape: [1, seq_len, hidden_size]

        // Mean Pooling
        let seq_len = view.shape()[1];
        let hidden_size = view.shape()[2];
        let mut sum = vec![0.0f32; hidden_size];

        for i in 0..seq_len {
            for j in 0..hidden_size {
                sum[j] += view[[0, i, j]];
            }
        }

        let mut mean: Vec<f32> = sum.into_iter().map(|s| s / (seq_len as f32)).collect();

        // Normalization (L2 Norm)
        let norm = mean.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for val in mean.iter_mut() {
                *val /= norm;
            }
        }

        Ok(mean)
    }
}
