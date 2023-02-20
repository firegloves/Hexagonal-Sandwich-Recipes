use crate::domain::sandwich::{Sandwich, SandwichType};

#[derive(Debug)]
pub enum CreateError {
    InvalidData(String),
    Unknown(String),
    Conflict(String)
}

// this is my port / use case
pub fn create_sandwich<'a>(name: &'a str, ingredients: &'a Vec<&str>, sandwich_type: &SandwichType) -> Result<Sandwich, CreateError> {

    let ingredients = ingredients.iter().map(|item| item.to_string()).collect::<Vec<String>>();
    let sandwich = Sandwich::new(String::from(""), name.to_string(), ingredients, sandwich_type.clone())
        .map_err(|e| CreateError::InvalidData(e))?;

    Ok(sandwich)
}


#[cfg(test)]
mod tests {
    use crate::helpers::string_vec_to_vec_str;
    use crate::tests::test_utils::shared::{assert_on_sandwich, SANDWICH_NAME, SANDWICH_TYPE, stub_ingredients, stub_sandwich};

    use super::*;

    #[test]
    fn should_create_the_expected_sandwich() {

        let ingredients = stub_ingredients();
        let ingredients = string_vec_to_vec_str(&ingredients);

        let sandwich = create_sandwich(SANDWICH_NAME, &ingredients, &SANDWICH_TYPE)
            .unwrap();

        assert_on_sandwich(sandwich, &stub_sandwich(false), false);
    }
}
