use std::sync::Arc;

use actix_web::web::Data;
use async_trait::async_trait;
use juniper::{EmptyMutation, EmptySubscription, FieldResult, graphql_object, GraphQLObject, RootNode};
use juniper_codegen::GraphQLEnum;
use serde::{Deserialize, Serialize};

use crate::domain;
use crate::domain::Entity;
use crate::domain::sandwich::{Sandwich, SandwichType};
use crate::driven::repository::{FindSandwich, RepoCreateError, RepoDeleteError, RepoFindAllError, RepoSelectError, Repository, RepoUpdateError};

#[derive(Clone, Debug, GraphQLObject)]
#[graphql(description="A sandwich recipe")]
pub struct SandwichGraphQL {
    pub id: String,
    pub name: String,
    pub ingredients: Vec<String>,
    pub sandwich_type: SandwichTypeGraphQL,
    pub stars: i32
}

impl From<Sandwich> for SandwichGraphQL {

    fn from(s: Sandwich) -> Self {

        let sand_graph = SandwichGraphQL {
            id: s.id().value().clone().unwrap(),
            name: s.name().value().to_string(),
            ingredients: s.ingredients().value().clone(),
            sandwich_type: SandwichTypeGraphQL::from(s.sandwich_type().clone()),
            stars: s.stars().value(),
        };

        sand_graph
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, GraphQLEnum)]
#[graphql(description="A sandwich type")]
pub enum SandwichTypeGraphQL {
    Meat,
    Fish,
    Veggie,
    Undefined
}

impl From<SandwichType> for SandwichTypeGraphQL {

    fn from(s: SandwichType) -> Self {

        match s {
            SandwichType::Meat => SandwichTypeGraphQL::Meat,
            SandwichType::Fish => SandwichTypeGraphQL::Fish,
            SandwichType::Veggie => SandwichTypeGraphQL::Veggie,
            SandwichType::Undefined => SandwichTypeGraphQL::Undefined
        }
    }
}

// This struct represents our context.
pub struct Context {
    pub repository: Arc<dyn Repository<Sandwich> + Send + Sync>,
}

// Mark the Context struct as a valid context type for Juniper
impl juniper::Context for Context {}

#[async_trait]
impl<T: ?Sized + Repository<U> + Sync + Send, U: Entity + Send + 'static> Repository<U> for Arc<T> {
    async fn create(&self, sandwich: U) -> Result<U, RepoCreateError> {
        (**self).create(sandwich).await
    }

    async fn find_one(&self, sandwich: FindSandwich) -> Result<U, RepoSelectError> {
        (**self).find_one(sandwich).await
    }

    async fn find_all(&self, sandwich: FindSandwich) -> Result<Vec<U>, RepoFindAllError> {
        (**self).find_all(sandwich).await
    }

    async fn update(&self, sandwich: U) -> Result<U, RepoUpdateError> {
        (**self).update(sandwich).await
    }

    async fn delete(&self, id: &str) -> Result<(), RepoDeleteError> {
        (**self).delete(id).await
    }
}

pub struct Query;

#[graphql_object(Context = Context)]
impl Query {

    #[graphql(description = "List of all sandwiches")]
    async fn sandwiches(context: &Context) -> FieldResult<Vec<SandwichGraphQL>> {

        let repository = context.repository.clone();

        let sandwiches = domain::find_all_sandwiches::find_all_sandwiches(Data::new(repository), "", &vec![]).await
            .expect("Error finding sandwiches");

        let res: Vec<SandwichGraphQL> = sandwiches.into_iter()
            .map(|sandwich| SandwichGraphQL::from(sandwich))
            .collect();

        Ok(res)
    }
}

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}