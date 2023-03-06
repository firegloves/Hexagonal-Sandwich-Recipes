use crate::domain::sandwich::Sandwich;

#[derive(Debug)]
pub enum FindAllError {
    Unknown(String)
}

// this is my port / use case
pub fn find_all_sandwiches<'a>(name: &'a str, ingredients: &'a Vec<&str>) -> Result<Vec<Sandwich>, FindAllError> {

    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_all_sandwiches() {

        let sand_list = find_all_sandwiches("", &vec![]).unwrap();

        assert_eq!(sand_list.len(), 0);
    }
}
