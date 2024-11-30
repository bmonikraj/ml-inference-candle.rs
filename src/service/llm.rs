use std::collections::HashMap;

pub trait Llm {
    fn load(&mut self, config: &HashMap<String, HashMap<String, String>>);
    fn set_writer(&mut self);
    fn generate_and_persist(&mut self);
}
