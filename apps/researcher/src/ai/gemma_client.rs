use crate::ai::tokenizer::GemmaTokenizer;
use anyhow::{Error, Result};
use candle_core::{Device, Tensor};

use candle_transformers::models::quantized_llama::ModelWeights as QLlama;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

pub struct Gemma1BClient {
    model: QLlama,
    tokenizer: GemmaTokenizer,
    device: Device,
}

impl Gemma1BClient {
    pub async fn new() -> Result<Self, Error> {
        // 1. Fetch Model file
        let model_data = fetch_bytes("models/gemma-3-1B-it-QAT-Q4_0.gguf").await?;

        // 2. Initialize Device (CPU for WASM, eventually WebGPU if supported)
        let device = Device::Cpu;

        // 3. Load Tokenizer (embedded at compile time)
        let tokenizer = GemmaTokenizer::from_embedded()?;

        // 4. Load Model
        let mut cursor = std::io::Cursor::new(model_data);
        let content = candle_core::quantized::gguf_file::Content::read(&mut cursor)?;
        let model = QLlama::from_gguf(content, &mut cursor, &device)?;

        // 5. Initialize Cache (Gemma Config)
        // Note: Gemma uses Llama architecture in Candle but with specific config
        // We might need to adjust these parameters for Gemma 1B specifically if defaults don't work

        Ok(Self {
            model,
            tokenizer,
            device,
        })
    }

    pub fn generate(&mut self, prompt: &str, max_tokens: usize) -> Result<String, Error> {
        let tokens = self.tokenizer.encode(prompt)?;

        let mut all_tokens = tokens.clone();
        let mut generated_tokens = Vec::new();
        let mut index_pos = 0;

        for _ in 0..max_tokens {
            let (context_tokens, context_index) = if index_pos == 0 {
                (all_tokens.clone(), 0)
            } else {
                (all_tokens[all_tokens.len() - 1..].to_vec(), index_pos)
            };

            let input = Tensor::new(context_tokens.as_slice(), &self.device)?.unsqueeze(0)?;
            let logits = self.model.forward(&input, context_index)?;
            let logits = logits.squeeze(0)?.squeeze(0)?;

            // Greedy sampling for now
            let next_token = logits.argmax(0)?.to_scalar::<u32>()?;

            all_tokens.push(next_token);
            generated_tokens.push(next_token);
            index_pos += context_tokens.len();

            if next_token == 1 || next_token == 107 {
                // EOS
                break;
            }
        }

        let output = self.tokenizer.decode(&generated_tokens)?;

        Ok(output)
    }
}

async fn fetch_bytes(url: &str) -> Result<Vec<u8>, Error> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)
        .map_err(|e| anyhow::anyhow!("Request creation failed: {:?}", e))?;

    let window = web_sys::window().ok_or_else(|| anyhow::anyhow!("No window"))?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| anyhow::anyhow!("Fetch failed: {:?}", e))?;

    let resp: Response = resp_value
        .dyn_into()
        .map_err(|_| anyhow::anyhow!("Response cast failed"))?;

    let buffer = JsFuture::from(
        resp.array_buffer()
            .map_err(|e| anyhow::anyhow!("ArrayBuffer failed: {:?}", e))?,
    )
    .await
    .map_err(|e| anyhow::anyhow!("ArrayBuffer future failed: {:?}", e))?;

    let bytes = js_sys::Uint8Array::new(&buffer).to_vec();
    Ok(bytes)
}
