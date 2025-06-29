mod print {
    pub fn print_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(filename)?;
        println!("{}", content);
        Ok(())
    }
}
