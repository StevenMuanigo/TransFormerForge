// Tokenizer implementation using tokenizers crate

use tokenizers::Tokenizer;
use anyhow::Result;
use std::path::Path;

pub struct CustomTokenizer {
    tokenizer: Option<Tokenizer>,
}

impl CustomTokenizer {
    pub fn new() -> Self {
        Self { tokenizer: None }
    }

    pub fn load_from_file(path: &Path) -> Result<Self> {
        let tokenizer = Tokenizer::from_file(path)
            .map_err(|e| anyhow::anyhow!("Failed to load tokenizer: {}", e))?;
        
        Ok(Self {
            tokenizer: Some(tokenizer),
        })
    }

    pub fn encode(&self, text: &str) -> Result<Vec<u32>> {
        if let Some(tokenizer) = &self.tokenizer {
            let encoding = tokenizer
                .encode(text, false)
                .map_err(|e| anyhow::anyhow!("Encoding failed: {}", e))?;
            
            Ok(encoding.get_ids().to_vec())
        } else {
            // Fallback: simple whitespace tokenization
            Ok(self.simple_tokenize(text))
        }
    }

    fn simple_tokenize(&self, text: &str) -> Vec<u32> {
        text.split_whitespace()
            .enumerate()
            .map(|(i, _)| i as u32)
            .collect()
    }

    pub fn decode(&self, ids: &[u32]) -> Result<String> {
        if let Some(tokenizer) = &self.tokenizer {
            tokenizer
                .decode(ids, false)
                .map_err(|e| anyhow::anyhow!("Decoding failed: {}", e))
        } else {
            Ok(format!("tokens: {:?}", ids))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokenize() {
        let tokenizer = CustomTokenizer::new();
        let tokens = tokenizer.encode("Hello World").unwrap();
        assert!(!tokens.is_empty());
    }
}
