#[derive(Debug)]
pub enum DeleteOneError {
    InvalidData(String),
    Unknown(String),
    NotFound
}

// this is my port / use case
pub fn delete_one_sandwich(id: &str) -> Result<(), DeleteOneError> {

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::tests::test_utils::shared::SANDWICH_ID;

    use super::*;

    #[test]
    fn should_delete_a_sandwich() {

        match delete_one_sandwich(SANDWICH_ID) {
            Ok(()) => {},
            _ => unreachable!()
        }
    }
}
