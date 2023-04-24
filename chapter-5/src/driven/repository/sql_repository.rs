// use std::str::FromStr;
//
// use async_trait::async_trait;
// use mongodb::{bson, Client, Collection};
// use mongodb::bson::{doc, Document};
// use mongodb::bson::oid::ObjectId;
// use mongodb::error::Error;
// use serde::{Deserialize, Serialize};
//
// use crate::config::PersistenceConfig;
// use crate::domain::sandwich::{RecipeType, Sandwich};
// use crate::driven::repository::{FindSandwich, RepoCreateError, RepoDeleteError, RepoFindAllError, RepoSelectError, Repository, RepoUpdateError};
//
// // TODO https://dev.to/hackmamba/build-a-rest-api-with-rust-and-sqldb-actix-web-version-ei1
// // #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
// #[derive(Debug, Serialize, Deserialize)]
// pub struct SandwichSql {
//     _id: ObjectId,
//     name: String,
//     ingredients: Vec<String>,
//     recipe_type: RecipeType
// }
//
// impl SandwichSql {
//
//     // move to official TryFrom
//     fn from(sandwich: &Sandwich) -> Result<Self, String> {
//
//         let object_id = match sandwich.id().value() {
//             Some(id) =>  ObjectId::from_str(id).unwrap(),
//             None => ObjectId::new()
//         };
//
//         let sand_sql = SandwichSql {
//             _id: object_id,
//             name: sandwich.name().value().to_string(),
//             ingredients: sandwich.ingredients().value().clone(),
//             recipe_type: sandwich.recipe_type().clone(),
//         };
//
//         Ok(sand_sql)
//     }
//
//     fn to_sandwich(&self) -> Result<Sandwich, String> {
//         let sandwich = Sandwich::new(self._id.to_string(),
//                                      self.name.to_string(),
//                                      self.ingredients.clone(),
//                                      self.recipe_type.clone())?;
//         Ok(sandwich)
//     }
// }
//
// #[derive(Clone)]
// pub struct SandwichSqlRepository {
//     host: String,
//     port: Option<u16>,
//     user: String,
//     password: String,
//     database: String,
//     collection: String,
//     conn_uri: String,
// }
//
// impl SandwichSqlRepository {
//     async fn open_connection(&self) -> Client {
//         let c = Client::with_uri_str(&self.conn_uri);
//         c.await.expect("Error while opening the connection with SqlDB")
//     }
//
//     async fn get_collection(&self) -> Collection<SandwichSql> {
//         let client = self.open_connection().await;
//         client.database(&self.database).collection(&self.collection)
//     }
//
//     // fn compose_document_from_map(&self, prop_map: HashMap<&str, &str>) -> Document {
//     //     if prop_map.contains_key("id") {
//     //         doc! {
//     //             "_id": bson::oid::ObjectId::parse_str(prop_map.get("id").unwrap()).unwrap()
//     //         }
//     //     } else {
//     //         // let bson_map: HashMap<String, Bson> = prop_map.into_iter().map(|(k, v)| (String::from(k), Bson::from(v))).collect();
//     //         // bson_map.into_iter().collect()
//     //         let mut doc = doc!{};
//     //         let mut doc2 = doc!{};
//     //         doc.insert("name", prop_map.get("name"));
//     //         doc.insert("ingredients", doc!{
//     //             "$all":
//     //         });
//     //         doc
//     //     }
//     // }
//
//     fn compose_document_from_sandwich(&self, sandwich: FindSandwich) -> Result<Document, Error> {
//         if sandwich.id.is_some() {
//             let id = sandwich.id.as_ref().unwrap();
//             Ok(
//                 doc! {
//                 "_id": bson::oid::ObjectId::parse_str(id).unwrap()
//             })
//         } else {
//             let mut doc = doc!{};
//
//             if ! sandwich.name.is_empty() {
//                 doc.insert("name", sandwich.name);
//             }
//
//             if ! sandwich.ingredients.is_empty() {
//                 doc.insert("ingredients", doc! {
//                     "$all": sandwich.ingredients
//                 });
//             }
//
//             Ok(doc)
//         }
//     }
// }
//
// #[async_trait]
// impl Repository<Sandwich> for SandwichSqlRepository {
//     fn new(config: &PersistenceConfig) -> Result<Self, String> where Self: Sized {
//         config.validate()?;
//         let config = config.clone();
//         let conn_uri = create_connection_uri(&config);
//
//         Ok(SandwichSqlRepository {
//             host: config.host,
//             port: config.port,
//             user: config.user,
//             password: config.password,
//             database: config.database,
//             collection: config.schema_collection,
//             conn_uri,
//         })
//     }
//
//     async fn create(&self, entity: Sandwich) -> Result<Sandwich, RepoCreateError> {
//
//         let sand_sql = match SandwichSql::from(&entity) {
//             Ok(s) => s,
//             Err(e) => return Err(RepoCreateError::InvalidData(e))
//         };
//
//         let serialized_entity = match bson::to_bson(&sand_sql) {
//             Err(e) => return Err(RepoCreateError::InvalidData(e.to_string())),
//             Ok(entity) => entity
//         };
//
//         let document = serialized_entity.as_document().unwrap();
//
//         // check if it's possible to use the shared function in this case too
//         let client = self.open_connection().await;
//         let recipes_coll = client.database(&self.database).collection(&self.collection);
//
//         let result = recipes_coll.insert_one(document.to_owned(), None).await;
//         let inserted_id = match result {
//             Ok(e) => e.inserted_id.as_object_id().unwrap(),
//             Err(e) => return Err(RepoCreateError::Unknown(e.to_string())),
//         };
//
//         let created = Sandwich::new(inserted_id.to_string(),
//                                     entity.name().value().to_string(),
//                                     entity.ingredients().value().clone(),
//                                     entity.recipe_type().clone())
//             .unwrap();
//         Ok(created)
//     }
//
//     // we use a map because in this way we can bypass entity validation
//     async fn find_one(&self, sandwich: FindSandwich) -> Result<Sandwich, RepoSelectError> {
//         let recipes_coll = self.get_collection().await;
//
//         let document = self.compose_document_from_sandwich(sandwich).unwrap();
//         let result: Result<Option<SandwichSql>, Error> = recipes_coll.find_one(document, None).await;
//
//         let found = match result {
//             Ok(s) => match s {
//                 Some(s) => s,
//                 None => return Err(RepoSelectError::NotFound)
//             },
//             Err(_) => return Err(RepoSelectError::Unknown(String::from("unknown error")))
//         };
//
//         let sandwich = match found.to_sandwich() {
//             Ok(s) => s,
//             Err(e) => return Err(RepoSelectError::Unknown(e.to_string())),
//         };
//
//         Ok(sandwich)
//     }
//
//     async fn find_all(&self, sandwich: FindSandwich) -> Result<Vec<Sandwich>, RepoFindAllError> {
//
//         let recipes_coll = self.get_collection().await;
//
//         let document = self.compose_document_from_sandwich(sandwich).unwrap();
//         let res = recipes_coll.find(document, None).await;
//
//         let mut cursor = match res {
//             Ok(c) => c,
//             Err(_) => return Ok(vec![])
//         };
//
//         let mut sand_vec: Vec<Sandwich> = Vec::new();
//
//         while cursor.advance().await
//             .map_err(|_ | RepoFindAllError::Unknown(String::from("Cursor iteration error")))? {
//
//             let sand = match cursor.deserialize_current() {
//                 Ok(s) => match s.to_sandwich() {
//                     Ok(s) => s,
//                     Err(s) => return Err(RepoFindAllError::Unknown(String::from(s)))
//                 },
//                 Err(_) => return Err(RepoFindAllError::Unknown(String::from("Error while deserializing")))
//             };
//             sand_vec.push(sand);
//         }
//
//         // while cursor.advance().await? {
//         //     let sand = cursor.deserialize_current()
//         //         .map_err(|e | Err(RepoSelectError::Unknown(String::from("Error while deserializing"))))?
//         //         .to_sandwich()
//         //         .map_err(|e | Err(RepoSelectError::Unknown(String::from(e))))?;
//         //
//         //     sand_vec.push(sand);
//         // }
//
//         Ok(sand_vec)
//     }
//
//
//     async fn update(&self, entity: &Sandwich) -> Result<Sandwich, RepoUpdateError> {
//
//         let sand_sql = match SandwichSql::from(&entity) {
//             Ok(s) => s,
//             Err(e) => return Err(RepoUpdateError::InvalidData(e))
//         };
//
//         let recipes_coll = self.get_collection().await;
//
//         let res = recipes_coll.update_one(
//             doc! {
//                 "_id": sand_sql._id
//             },
//             doc! {
//                 "$set": {
//                     "name": sand_sql.name,
//                     "ingredients": sand_sql.ingredients
//                 }
//             },
//             None
//         ).await;
//
//         return match res {
//             Ok(ur) => {
//                 if ur.modified_count > 0 {
//                     Ok(entity.to_owned())
//                 } else {
//                     Err(RepoUpdateError::Unknown(format!("The update modified {} documents", ur.modified_count)))
//                 }
//             },
//             Err(_) => {
//                 Err(RepoUpdateError::Unknown(String::from("An error occurred while updating the document")))
//             }
//         }
//     }
//
//     async fn delete(&self, id: &str) -> Result<(), RepoDeleteError> {
//
//         let object_id = match ObjectId::from_str(id) {
//             Ok(id) => id,
//             Err(e) => return Err(RepoDeleteError::InvalidData(e.to_string()))
//         };
//
//         let recipes_coll = self.get_collection().await;
//
//         let res = recipes_coll.delete_one(
//             doc! {
//                 "_id": object_id
//             },
//             None
//         ).await;
//
//         match res {
//             Ok(dr) => {
//                 if dr.deleted_count == 1 {
//                     Ok(())
//                 } else {
//                     Err(RepoDeleteError::NotFound)
//                 }
//             }
//             Err(_) => Err(RepoDeleteError::Unknown(String::from("An error occurred during the deletion")))
//         }
//     }
// }
//
//
// fn create_connection_uri(config: &PersistenceConfig) -> String {
//     format!("sqldb://{}:{}@{}/{}",
//             config.user,
//             config.password,
//             match config.port {
//                 None => String::from("aoaoaoao"),
//                 Some(port) => config.host.clone()
//                     + ":"
//                     + &port.to_string(),
//             },
//             "admin") // FIXME this data must be dynamic
// }
//
//
// #[cfg(test)]
// mod tests {
//     use std::collections::HashMap;
//
//     use crate::config::parse_local_config;
//     use crate::test::{assert_on_sandwich, SANDWICH_ID, SANDWICH_NAME, stub_sandwich};
//
//     use super::*;
//
//     #[test]
//     fn should_create_the_expected_sqldb_repo() {
//         let config = stub_config();
//         let repo: SandwichSqlRepository = Repository::<Sandwich>::new(&config).unwrap();
//         assert_eq!(repo.conn_uri, "sqldb://myuser:mypass@myhost:27777/admin");
//     }
//
//     #[test]
//     fn should_return_error_with_an_invalid_config() {
//         let mut config = stub_config();
//         config.host = "".to_string();
//         let result: Result<SandwichSqlRepository, String> = Repository::<Sandwich>::new(&config);
//         assert_eq!(result.is_err(), true);
//
//         // implement other test cases
//     }
//
//     #[actix_rt::test]
//     async fn should_create_a_sandwich() {
//         let config = parse_local_config();
//         let repo: SandwichSqlRepository = Repository::<Sandwich>::new(&config.persistence).unwrap();
//
//         let res = repo.create(stub_sandwich(false)).await;
//         let expected = stub_sandwich(false);
//
//         match_and_assert_on_sandwich(res, expected);
//         // clean_db(repo).await;
//     }
//
//     #[actix_rt::test]
//     async fn should_find_one_sandwich() {
//         let config = parse_local_config();
//         let repo: SandwichSqlRepository = Repository::<Sandwich>::new(&config.persistence).unwrap();
//
//         repo.create(stub_sandwich(false)).await.unwrap();
//
//         // let prop_map = stub_prop_map();
//         // let res = repo.find_one(prop_map).await;
//         //
//         // let expected = stub_sandwich(false);
//         // match_and_assert_on_sandwich(res, expected);
//
//         //
//
//         // let mut prop_map: HashMap<&str, &str> = HashMap::new();
//         // prop_map.insert("ingredients", "Wurst");
//         // let res = repo.find_one(prop_map).await;
//
//         let s = FindSandwich {
//             id: None,
//             name: String::from(""),
//             ingredients: vec![String::from("Wurst")]
//         };
//
//         let res = repo.find_one( s).await;
//
//         let expected = stub_sandwich(false);
//         match_and_assert_on_sandwich(res, expected);
//
//         // clean_db(repo).await;
//     }
//
//     // #[actix_rt::test]
//     // async fn should_find_one_sandwich() {
//     //     let config = parse_local_config();
//     //     let repo: SandwichSqlRepository = Repository::<Sandwich>::new(&config.persistence).unwrap();
//     //
//     //     let recipes_coll = repo.get_collection().await;
//     //
//     //     let s = FindSandwich {
//     //         id: None,
//     //         name: String::from(""),
//     //         ingredients: vec![String::from("Wurst")]
//     //     };
//     //
//     //
//     //     let document = doc! {
//     //             "ingredients": { "$all": ["Wurst"]}
//     //         };
//     //     let result: Result<Option<SandwichSql>, Error> = recipes_coll.find_one(document, None).await;
//     //
//     //     let result = result.unwrap().unwrap().to_sandwich();
//     //
//     //     assert_eq!(result.is_ok(), true);
//     //
//     // }
//
//     // FIXME
//     // #[actix_rt::test]
//     // async fn should_update_a_sandwich() {
//     //     let config = parse_local_config();
//     //     let repo: SandwichSqlRepository = Repository::<Sandwich>::new(&config.persistence).unwrap();
//     //
//     //     let sandwich = stub_sandwich(false);
//     //     let created = repo.create(sandwich).await.unwrap();
//     //
//     //     let new_name = "Hamburger";
//     //     let new_ingredients = vec![String::from("Meath"), String::from("Ketchup"), String::from("Mayo")];
//     //     let updating_sandwich = Sandwich::new(created.id().as_ref().unwrap().to_string(), String::from(new_name), new_ingredients.clone()).unwrap();
//     //
//     //     let res = repo.update(&updating_sandwich).await;
//     //
//     //     match_and_assert_on_sandwich(res, updating_sandwich);
//     //     clean_db(repo).await;
//     // }
//
//     // #[test]
//     // fn should_compose_the_expected_doc() {
//     //     let config = parse_local_config();
//     //     let repo: SandwichSqlRepository = Repository::<Sandwich>::new(&config.persistence).unwrap();
//     //
//     //     let prop_map = stub_prop_map();
//     //     let doc = repo.compose_document_from_map(prop_map);
//     //
//     //     assert_eq!(doc.get("name").unwrap().to_string(), ["\"", SANDWICH_NAME, "\""].join(""));
//     // }
//
//     #[actix_rt::test]
//     async fn should_delete_a_doc_by_id() {
//         let config = parse_local_config();
//         let repo: SandwichSqlRepository = Repository::<Sandwich>::new(&config.persistence).unwrap();
//
//         let created = repo.create(stub_sandwich(false)).await.unwrap();
//
//         let res = repo.delete(created.id().as_ref().unwrap()).await;
//         assert!(res.is_ok());
//         clean_db(repo).await;
//     }
//
//     fn stub_config() -> PersistenceConfig {
//         PersistenceConfig {
//             host: "myhost".to_string(),
//             port: Some(27777),
//             user: "myuser".to_string(),
//             password: "mypass".to_string(),
//             database: "mydb".to_string(),
//             schema_collection: "mycoll".to_string(),
//         }
//     }
//
//     fn stub_prop_map() -> HashMap<&'static str, &'static str> {
//         let mut map: HashMap<&str, &str> = HashMap::new();
//         map.insert("name", SANDWICH_NAME);
//         map
//     }
//
//     fn match_and_assert_on_sandwich<T>(res: Result<Sandwich, T>, expected: Sandwich) {
//         match res {
//             Ok(sandwich) => {
//                 assert!(sandwich.id().is_some());
//                 assert_on_sandwich(expected, &sandwich, false);
//             },
//             _ => unreachable!()
//         };
//     }
//
//     async fn clean_db(repo: SandwichSqlRepository) {
//         let recipes_coll = repo.get_collection().await;
//         let x = recipes_coll.delete_many(doc! {}, None).await.unwrap();
//     }
// }
