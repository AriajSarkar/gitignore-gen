use reqwest::blocking::get;
use std::error::Error;

/// Fetches a gitignore template from the toptal API for the given technologies
pub fn fetch_gitignore_template(technologies: &[String]) -> Result<String, Box<dyn Error>> {
    let tech_list = technologies.join(",");
    let url = format!("https://www.toptal.com/developers/gitignore/api/{}", tech_list);

    let response = get(&url)?;
    let content = response.text()?;

    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_gitignore_template() {
        let technologies = vec!["Go".to_string(), "Node".to_string()];
        let content = fetch_gitignore_template(&technologies);

        assert!(content.is_ok());
        let content = content.unwrap();
        assert!(!content.is_empty());
    }
}
