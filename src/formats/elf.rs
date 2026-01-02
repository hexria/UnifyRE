use crate::errors::Result;
use object::{Object, ObjectSection, elf};

pub struct ElfAnalyzer<'a> {
    file: &'a object::File<'a>,
}

impl<'a> ElfAnalyzer<'a> {
    pub fn new(file: &'a object::File<'a>) -> Self {
        Self { file }
    }

    pub fn extract_metadata(&self) -> Result<serde_json::Value> {
        let mut metadata = serde_json::Value::Object(serde_json::Map::new());

        if let Some(obj) = metadata.as_object_mut() {
            obj.insert(
                "type".to_string(),
                serde_json::Value::String("ELF".to_string()),
            );

            let mut sections = Vec::new();
            for section in self.file.sections() {
                sections.push(section.name().unwrap_or_default().to_string());
            }
            obj.insert(
                "sections_count".to_string(),
                serde_json::Value::Number(sections.len().into()),
            );
        }

        Ok(metadata)
    }
}
