pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        } 
    }
    return results;
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let new_query = query.to_lowercase();
    for line in content.lines() {
        let current = line;
        if line.contains(&new_query) {
            results.push(current)
        }
    }
    return results;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive,
Pick three.";
        
        assert_eq!(vec!["safe, fast, productive,"], search_case_insensitive(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
Duct tape.";
        assert_eq!(
            vec!["Trust me."],
            search_case_sensitive(query, contents))
    }
}