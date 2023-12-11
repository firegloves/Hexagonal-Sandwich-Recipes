use actix_web::{HttpResponse, web};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use serde_qs::actix::QsQuery;
use validator::Validate;

use crate::{domain, Repository, Sandwich};
use crate::domain::create_sandwich::CreateError;
use crate::domain::delete_one_sandwich::DeleteOneError;
use crate::domain::find_all_sandwiches::FindAllError;
use crate::domain::find_one_sandwich::FindOneError;
use crate::domain::sandwich::SandwichType;
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
// #[derive(Debug, Deserialize, Serialize)]
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

/// find sandwich recipes
pub async fn find_sandwiches<T: Repository<Sandwich>>(
    repository: web::Data<T>,
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
    let result = domain::find_all_sandwiches::find_all_sandwiches(repository, name, &ingredients).await;

    result
        .map(|v| respond_json(SandwichListResponse::from(v)))
        .map_err(|e| match e {
            FindAllError::Unknown(m) => ApiError::Unknown(m),
        })?
}

/// get by id
pub async fn get_by_id<T: Repository<Sandwich>>(
    repository: web::Data<T>,
    path: web::Path<String>,
) -> Result<Json<SandwichResponse>, ApiError> {

    let sandwich_id = path.into_inner();

    let result = domain::find_one_sandwich::find_one_sandwich(
        repository,
        sandwich_id.as_str(),
        "",
        vec![].as_ref()).await;

    result
        .map(|v| respond_json(SandwichResponse::from(v)))
        .map_err(|e| match e {
            FindOneError::Unknown(m) => ApiError::Unknown(m),
            FindOneError::NotFound => ApiError::NotFound(String::from("No sandwich found with the specified criteria")),
        })?
}

/// create sandwich recipes
pub async fn create_sandwich<T: Repository<Sandwich>>(
    repository: web::Data<T>,
    request: Json<CreateSandwichRequest>,
) -> Result<Json<SandwichResponse>, ApiError> {

    validate(&request)?;

    let result = domain::create_sandwich::create_sandwich(
        repository,
        &request.name,
        string_vec_to_vec_str(&request.ingredients).as_ref(),
        &request.sandwich_type).await;

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
pub async fn update_sandwich<T: Repository<Sandwich>>(
    repository: web::Data<T>,
    request: Json<UpdateSandwichRequest>,
) -> Result<Json<SandwichResponse>, ApiError> {

    validate(&request)?;

    let result = domain::update_sandwich::update_sandwich(
        repository,
        request.id.as_str(),
        request.name.as_str(),
        string_vec_to_vec_str(&request.ingredients).as_ref(),
        &request.sandwich_type).await;

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
pub async fn delete_one_sandwich<T: Repository<Sandwich>>(
    repository: web::Data<T>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let sandwich_id = path.into_inner();

    let result = domain::delete_one_sandwich::delete_one_sandwich(
        repository,
        sandwich_id.as_str()).await;

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
    use actix_web::web::Data;
    use serial_test::serial;

    use crate::driven::repository::mongo_repository::SandwichMongoRepository;
    use crate::tests::test_utils::shared;
    use crate::tests::test_utils::shared::{assert_on_ingredients, CHEESEBURGER_NAME, create_default_sandwich, delete_sandwich_from_sandwich_response, delete_sandwiches_from_list_response, empty_find_sandwich, get_testing_persistence_config, SANDWICH_NAME, SANDWICH_TYPE, stub_cheeseburger, stub_cheeseburger_ingredients, stub_ingredients, stub_sandwich};

    use super::*;

    #[serial]
    #[actix_web::test]
    async fn should_find_all_sandwiches() {
        let repo = SandwichMongoRepository::new(&get_testing_persistence_config()).unwrap();
        create_default_sandwich(&repo).await;
        shared::create_sandwich(&repo, stub_cheeseburger()).await;

        let resp: SandwichListResponse = execute::<>(&repo,
                                                     "/",
                                                     None,
                                                     web::get(),
                                                     TestRequest::get(),
                                                     find_sandwiches::<SandwichMongoRepository>,
                                                     None::<FindSandwichRequest>)
            .await;

        assert_eq!(resp.sandwiches.len(), 2);
        assert_on_sandwich_response(&resp.sandwiches[0], &stub_sandwich(false));
        assert_on_sandwich_response(&resp.sandwiches[1], &stub_cheeseburger());

        delete_sandwiches_from_list_response(&repo, resp.sandwiches).await;
    }

    #[serial]
    #[actix_web::test]
    async fn should_create_a_sandwich() {
        let repo = SandwichMongoRepository::new(&get_testing_persistence_config()).unwrap();

        let create_req = CreateSandwichRequest {
            name: SANDWICH_NAME.to_string(),
            ingredients: stub_ingredients(),
            sandwich_type: SANDWICH_TYPE,
        };

        let resp = execute::<>(&repo,
                               "/",
                               None,
                               web::post(),
                               TestRequest::post(),
                               create_sandwich::<SandwichMongoRepository>,
                               Some(create_req))
            .await;

        assert_on_sandwich_response(&resp, &stub_sandwich(false));

        delete_sandwich_from_sandwich_response(&repo, &resp).await;
    }

    #[serial]
    #[actix_web::test]
    async fn should_update_a_sandwich() {
            let repo = SandwichMongoRepository::new(&get_testing_persistence_config()).unwrap();
            let sandwich = create_default_sandwich(&repo).await;

            let updt_req = UpdateSandwichRequest {
            id: sandwich.id().value().as_ref().unwrap().to_string(),
            name: CHEESEBURGER_NAME.to_string(),
            ingredients: stub_cheeseburger_ingredients(),
            sandwich_type: SandwichType::Veggie,
        };
        let expected = Sandwich::new(updt_req.id.clone(), updt_req.name.clone(), updt_req.ingredients.clone(), updt_req.sandwich_type.clone()).unwrap();

        let resp = execute::<>(&repo,
                               "/",
                               None,
                               web::put(),
                               TestRequest::put(),
                               update_sandwich::<SandwichMongoRepository>,
                               Some(updt_req))
            .await;

        assert_on_sandwich_response(&resp, &expected);

        delete_sandwich_from_sandwich_response(&repo, &resp).await;
    }

    #[serial]
    #[actix_web::test]
    async fn should_find_a_sandwich_by_id() {

        let repo = SandwichMongoRepository::new(&get_testing_persistence_config()).unwrap();
        let sandwich = create_default_sandwich(&repo).await;
        let uri_to_call = format!("/{}", sandwich.id().value().as_ref().unwrap());

        let resp = execute(&repo,
                           "/{id}",
                           Some(&uri_to_call),
                           web::get(),
                           TestRequest::get(),
                           get_by_id::<SandwichMongoRepository>,
                           None::<String>)
            .await;

        assert_on_sandwich_response(&resp, &sandwich);

        delete_sandwich_from_sandwich_response(&repo, &resp).await;
    }

    #[serial]
    #[actix_web::test]
    async fn should_delete_a_sandwich() {

        let repo = SandwichMongoRepository::new(&get_testing_persistence_config()).unwrap();
        let sandwich = create_default_sandwich(&repo).await;

        let sand_list = repo.find_all(empty_find_sandwich()).await.unwrap();
        assert_eq!(1, sand_list.len());

        let uri_to_call = format!("/{}", sandwich.id().value().as_ref().unwrap());

        let app = test::init_service(
            App::new()
                .app_data(Data::new(repo.clone()))
                .route("/{id}", web::delete().to(delete_one_sandwich::<SandwichMongoRepository>))).await;
        let req = TestRequest::delete()
            .uri(&uri_to_call)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let sand_list = repo.find_all(empty_find_sandwich()).await.unwrap();
        assert_eq!(0, sand_list.len());
    }


    /// execute a test request
    async fn execute<F, Args, R, Ret>(repo: &R, path: &str, uri_to_call: Option<&str>, http_method: Route, test_req: TestRequest, handler: F, recipe_req: Option<impl Serialize>) -> Ret
        where
            R: Repository<Sandwich> + Send + Sync + 'static + Clone,
            F: Handler<Args>,
            Args: FromRequest + 'static,
            F::Output: Responder,
            Ret: for<'de> Deserialize<'de> {

        // init service
        let app = test::init_service(
            App::new()
                .app_data(Data::new(repo.clone()))
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
