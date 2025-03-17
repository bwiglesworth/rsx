use std::path::PathBuf;

pub struct Builder {
    source_dir: PathBuf,
    output_dir: PathBuf,
    assets: Vec<Asset>,
}

#[derive(Clone)]
pub struct Asset {
    pub path: PathBuf,
    pub content: Vec<u8>,
}

impl Builder {
    pub fn new(source_dir: PathBuf) -> Self {
        Builder {
            source_dir,
            output_dir: PathBuf::from("dist"),
            assets: Vec::new(),
        }
    }

    pub fn compile(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Scan source directory
        self.scan_sources()?;
        
        // Process assets
        self.process_assets()?;
        
        // Generate optimized output
        self.generate_output()?;
        
        Ok(())
    }

    fn scan_sources(&mut self) -> Result<(), std::io::Error> {
        // TODO: Implement source scanning
        Ok(())
    }

    fn process_assets(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement asset processing
        Ok(())
    }

    fn generate_output(&self) -> Result<(), std::io::Error> {
        // TODO: Implement output generation
        Ok(())
    }

    pub fn get_assets(&self) -> &[Asset] {
        &self.assets
    }

    pub fn compile_assets(&mut self) -> &[Asset] {
        let output_dir = &self.output_dir;
        self.assets = std::fs::read_dir(&self.source_dir)
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                let path = entry.path();
                let content = std::fs::read(&path).unwrap();
                let relative_path = path.strip_prefix(&self.source_dir).unwrap();
                
                // Write to output directory
                let output_path = output_dir.join(relative_path);
                std::fs::create_dir_all(output_path.parent().unwrap()).unwrap();
                std::fs::write(&output_path, &content).unwrap();
                
                Asset {
                    path: relative_path.to_path_buf(),
                    content,
                }
            })
            .collect();

        &self.assets    }}