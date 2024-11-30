use super::llm::Llm;

pub struct LLMQwen {

}

impl Llm for LLMQwen {
    fn load(&mut self, config: &std::collections::HashMap<String, std::collections::HashMap<String, String>>) {
        log::info!("LLMQwen invoked for loading")
    }

    fn set_writer(&mut self) {
        log::info!("LLMQwen invoked for setting writer")
    }

    fn generate_and_persist(&mut self) {
        log::info!("LLMQwen invoked for generation")
    }
}

impl LLMQwen {
    pub fn new() -> Self {
        return Self {};
    }
}