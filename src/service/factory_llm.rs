use std::error::Error;

use super::{llm::Llm, llm_qwen::LLMQwen, llm_smol::LLMSmol};

pub fn get_llm(type_of_llm: &String) -> Result<Box<dyn Llm>, Box<dyn Error>> {
    match type_of_llm.as_str() {
        "smol" => Ok(Box::new(LLMSmol::new())),
        "qwen" => Ok(Box::new(LLMQwen::new())),
        _ => Err(format!("{} is not a valid LLM type", type_of_llm).into()),
    }
}
