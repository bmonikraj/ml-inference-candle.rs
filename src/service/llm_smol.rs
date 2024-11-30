use super::llm::Llm;

pub struct LLMSmol {

}

impl Llm for LLMSmol {
    fn load(&mut self, config: &std::collections::HashMap<String, std::collections::HashMap<String, String>>) {
        log::info!("LLMSmol invoked for loading")
    }

    fn generate(&mut self) {
        log::info!("LLMSmol invoked for generation")
    }
}

impl LLMSmol {
    pub fn new() -> Self {
        return Self {};
    }
}