use super::llm::Llm;

pub struct LLMQwen {

}

impl Llm for LLMQwen {
    fn load(&mut self, config: &std::collections::HashMap<String, std::collections::HashMap<String, String>>) {
        log::info!("LLMQwen invoked for loading")
    }

    fn generate(&mut self) {
        log::info!("LLMQwen invoked for generation")
    }
}

impl LLMQwen {
    pub fn new() -> Self {
        return Self {};
    }
}