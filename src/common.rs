use std::fs;

pub fn read_file2(file_name: &str) -> Result<std::vec::Vec<String>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(file_name)?;
    Ok(contents.split("\n")
        .map(|s| s.to_string())
        .collect())
}

pub fn read_file(file_name: &str) -> Result<std::vec::Vec<String>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(file_name)?;
    Ok(contents.split("\n")
        .filter(|x| !x.is_empty())
        .map(|s| s.to_string())
        .collect())
}

