use anyhow::Result;
use crate::config::AppConfig;

pub mod tokenizer;
pub mod normalizer;

pub fn preprocess_text(input: &str, config: &AppConfig) -> Result<String> {
    let mut text = input.to_string();

    // Normalize
    if config.preprocessing.lowercase {
        text = text.to_lowercase();
    }

    if config.preprocessing.remove_special_chars {
        text = normalizer::remove_special_characters(&text);
    }

    // Truncate if too long
    if text.len() > config.preprocessing.max_input_length {
        text.truncate(config.preprocessing.max_input_length);
    }

    // Additional normalization
    text = normalizer::normalize_whitespace(&text);
    text = normalizer::remove_extra_spaces(&text);

    Ok(text)
}
