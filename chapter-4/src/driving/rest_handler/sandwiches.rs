use serde::{Deserialize, Serialize};

use actix_web::{HttpResponse, web};
use actix_web::web::Json;
use serde_qs::actix::QsQuery;
use validator::Validate;

use crate::domain;
use crate::domain::create_sandwich::CreateError;
use crate::domain::delete_one_sandwich::DeleteOneError;
use crate::domain::find_all_sandwiches::FindAllError;
use crate::domain::find_one_sandwich::FindOneError;
use crate::domain::sandwich::{Sandwich, SandwichType};
use crate::domain::update_sandwich::UpdateError;
use crate::driving::rest_handler::errors::ApiError;
use crate::driving::rest_handler::validate::validate;
use crate::helpers::{respond_json, string_vec_to_vec_str};

//
// REQUESTS
//

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateSandwichRequest {
    #[validate(length(
    min = 3,
    message = "name is required and must be at least 3 characters"
    ))]
    pub name: String,

    #[validate(length(
    min = 1,
    message = "ingredients is required and must be at least 1 item"
    ))]
    pub ingredients: Vec<String>,

    pub sandwich_type: SandwichType,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateSandwichRequest {
    #[validate(length(
    min = 5,
    message = "id is required and must be at least 5 characters"
    ))]
    pub id: String,

    #[validate(length(
    min = 3,
    message = "name is required and must be at least 3 characters"
    ))]
    pub name: String,

    #[validate(length(
    min = 1,
    message = "ingredients is required and must be at least 1 item"
    ))]
    pub ingredients: Vec<String>,

    pub sandwich_type: SandwichType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindSandwichRequest {
    pub name: Option<String>,

    pub ingredients: Option<Vec<String>>,

    pub sandwich_type: Option<SandwichType>,
}

//
// RESPONSES
//

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SandwichResponse {
    pub id: String,
    pub name: String,
    pub ingredients: Vec<String>,
    pub sandwich_type: SandwichType,
}

impl From<Sandwich> for SandwichResponse {
    fn from(s: Sandwich) -> Self {
        SandwichResponse {
            id: s.id().value().clone().unwrap_or(String::from("")).to_string(),
            name: s.name().value().to_string(),
            ingredients: s.ingredients().value().clone(),
            sandwich_type: s.sandwich_type().clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SandwichListResponse {
    sandwiches: Vec<SandwichResponse>,
}

impl From<Vec<Sandwich>> for SandwichListResponse {
    fn from(v: Vec<Sandwich>) -> Self {
        let sandwiches = v.into_iter()
            .map(|s| SandwichResponse::from(s)
            ).collect();

        SandwichListResponse {
            sandwiches
        }
    }
}

/// find all sandwich recipes
pub async fn find_sandwiches(
    find_req: QsQuery<FindSandwichRequest>,
) -> Result<Json<SandwichListResponse>, ApiError> {
    let name = match &find_req.name {
        Some(n) => n.as_str(),
        None => ""
    };

    let ingredients = match &find_req.ingredients {
        Some(i) => string_vec_to_vec_str(&i),
        None => vec![]
    };
    let result = domain::find_all_sandwiches::find_all_sandwiches(name, &ingredients);

    result
        .map(|v| respond_json(SandwichListResponse::from(v)))
        .map_err(|e| match e {
            FindAllError::Unknown(m) => ApiError::Unknown(m),
        })?
}

/// get by id
pub async fn get_by_id(
    path: web::Path<String>,
) -> Result<Json<SandwichResponse>, ApiError> {

    let sandwich_id = path.into_inner();

    let result = domain::find_one_sandwich::find_one_sandwich(
        sandwich_id.as_str(),
        "",
        vec![].as_ref());

    result
        .map(|v| respond_json(SandwichResponse::from(v)))
        .map_err(|e| match e {
            FindOneError::Unknown(m) => ApiError::Unknown(m),
            FindOneError::NotFound => ApiError::NotFound(String::from("No sandwich found with the specified criteria")),
        })?
}

/// create sandwich recipes
pub async fn create_sandwich(
    request: Json<CreateSandwichRequest>,
) -> Result<Json<SandwichResponse>, ApiError> {

    validate(&request)?;

    let result = domain::create_sandwich::create_sandwich(
        &request.name,
        string_vec_to_vec_str(&request.ingredients).as_ref(),
        &request.sandwich_type);

    result
        .map(|v| {
            respond_json(SandwichResponse::from(v))
        })
        .map_err(|e| match e {
            CreateError::Unknown(m) => ApiError::Unknown(m),
            CreateError::InvalidData(m) => ApiError::InvalidData(m),
            CreateError::Conflict(m) => ApiError::Conflict(m)
        })?
}

/// update sandwich recipes
pub async fn update_sandwich(
    request: Json<UpdateSandwichRequest>,
) -> Result<Json<SandwichResponse>, ApiError> {

    validate(&request)?;

    let result = domain::update_sandwich::update_sandwich(
        request.id.as_str(),
        request.name.as_str(),
        string_vec_to_vec_str(&request.ingredients).as_ref(),
        &request.sandwich_type);

    result
        .map(|v| respond_json(SandwichResponse::from(v)))
        .map_err(|e| match e {
            UpdateError::Unknown(m) => ApiError::Unknown(m),
            UpdateError::InvalidData(m) => ApiError::InvalidData(m),
            UpdateError::NotFound => ApiError::NotFound(String::from("No sandwich to update corresponding to the specified criteria")),
            UpdateError::Conflict(m) => ApiError::Conflict(m)
        })?
}


/// delete one sandwich recipes
pub async fn delete_one_sandwich(
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let sandwich_id = path.into_inner();

    let result = domain::delete_one_sandwich::delete_one_sandwich(sandwich_id.as_str());

    result
        .map(|_| Ok(HttpResponse::Ok().finish()))
        .map_err(|e| match e {
            DeleteOneError::Unknown(m) => ApiError::Unknown(m),
            DeleteOneError::InvalidData(m) => ApiError::BadRequest(m),
            DeleteOneError::NotFound => ApiError::NotFound(String::from("No sandwich to delete corresponding with the received id"))
        })?
}

#[cfg(test)]
mod tests {
    use actix_web::{App, FromRequest, Handler, Responder, Route, test};
    use actix_web::test::TestRequest;

    use crate::tests::test_utils::shared::{assert_on_ingredients, CHEESEBURGER_NAME, SANDWICH_ID, SANDWICH_NAME, SANDWICH_TYPE, stub_cheeseburger_ingredients, stub_ingredients, stub_sandwich};

    use super::*;

    #[actix_web::test]
    async fn should_find_all_sandwiches() {

        let resp: SandwichListResponse = execute::<>("/",
                                                     None,
                                                     web::get(),
                                                     TestRequest::get(),
                                                     find_sandwiches,
                                                     None::<FindSandwichRequest>)
            .await;

        assert_eq!(resp.sandwiches.len(), 0);
    }

    #[actix_web::test]
    async fn should_create_a_sandwich() {

        let create_req = CreateSandwichRequest {
            name: SANDWICH_NAME.to_string(),
            ingredients: stub_ingredients(),
            sandwich_type: SANDWICH_TYPE,
        };

        let resp = execute::<>("/",
                               None,
                               web::post(),
                               TestRequest::post(),
                               create_sandwich,
                               Some(create_req))
            .await;

        assert_on_sandwich_response(&resp, &stub_sandwich(false));
    }

    #[actix_web::test]
    async fn should_update_a_sandwich() {
        let sandwich = stub_sandwich(true);

        let updt_req = UpdateSandwichRequest {
            id: sandwich.id().value().as_ref().unwrap().to_string(),
            name: CHEESEBURGER_NAME.to_string(),
            ingredients: stub_cheeseburger_ingredients(),
            sandwich_type: SandwichType::Veggie,
        };
        let expected = Sandwich::new(updt_req.id.clone(), updt_req.name.clone(), updt_req.ingredients.clone(), updt_req.sandwich_type.clone()).unwrap();

        let resp = execute::<>("/",
                               None,
                               web::put(),
                               TestRequest::put(),
                               update_sandwich,
                               Some(updt_req))
            .await;

        assert_on_sandwich_response(&resp, &expected);
    }

    // at the moment this test does fail, we will fix it later
    #[actix_web::test]
    async fn should_find_a_sandwich_by_id() {

        let sandwich= stub_sandwich(true);

        let uri_to_call = format!("/{}", sandwich.id().value().as_ref().unwrap());

        let resp = execute("/{id}",
                           Some(&uri_to_call),
                           web::get(),
                           TestRequest::get(),
                           get_by_id,
                           None::<String>)
            .await;

        assert_on_sandwich_response(&resp, &sandwich);
    }


    #[actix_web::test]
    async fn should_delete_a_sandwich() {

        let uri_to_call = format!("/{}", SANDWICH_ID);

        let app = test::init_service(
            App::new()
                .route("/{id}", web::delete().to(delete_one_sandwich))).await;
        let req = TestRequest::delete()
            .uri(&uri_to_call)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }


    /// execute a test request
    async fn execute<F, Args, Ret>(path: &str, uri_to_call: Option<&str>, http_method: Route, test_req: TestRequest, handler: F, recipe_req: Option<impl Serialize>) -> Ret
        where
            F: Handler<Args>,
            Args: FromRequest + 'static,
            F::Output: Responder,
            Ret: for<'de> Deserialize<'de> {

        // init service
        let app = test::init_service(
            App::new()
                .route(path, http_method.to(handler))).await;

        // set uri
        let req = match uri_to_call {
            Some(uri) => test_req.uri(uri),
            None => test_req
        };

        // Set json body
        let req = match recipe_req {
            Some(ref r) => req.set_json(recipe_req.unwrap()),
            None => req
        };

        test::call_and_read_body_json(&app, req.to_request()).await
    }

    fn assert_on_sandwich_response(actual: &SandwichResponse, expected: &Sandwich) {
        assert_eq!(&actual.name, expected.name().value());
        assert_on_ingredients(&actual.ingredients, expected.ingredients().value());
    }
}
