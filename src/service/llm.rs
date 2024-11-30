use std::collections::HashMap;

pub trait Llm {
    fn load(&mut self, config: &HashMap<String, HashMap<String, String>>);
    fn generate(&mut self);
}
