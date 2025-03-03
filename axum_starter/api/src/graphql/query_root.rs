use async_graphql::dynamic::*;
use axum_example_service::sea_orm;
use axum_example_service::sea_orm::DatabaseConnection;
use entity::post;
use seaography::{async_graphql, lazy_static, Builder, BuilderContext};

seaography::register_entity_modules!([post]);

lazy_static::lazy_static! {
    static ref CONTEXT: BuilderContext = BuilderContext::default();
}

pub fn schema(
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    // Construct GraphQL schema
    let builder = Builder::new(&CONTEXT, database.clone());
    let builder = register_entity_modules(builder);
    builder
        // Maximum depth of the constructed query
        .set_depth_limit(depth)
        // Maximum complexity of the constructed query
        .set_complexity_limit(complexity)
        .schema_builder()
        // GraphQL schema with database connection
        .data(database)
        .finish()
}
