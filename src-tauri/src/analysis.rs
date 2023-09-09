#[derive(Default, Clone)]
pub struct Analysis {
    pub data: Vec<String>,
}

impl Analysis {
    pub fn new() -> Analysis {
        Analysis::default()
    }

    pub fn read_hash(&mut self, files: &Vec<String>) {
        for file in files.clone() {
            self.data.push(file);
        }
    }

    pub fn data(&self) -> &Vec<String> {
        &self.data
    }
}
