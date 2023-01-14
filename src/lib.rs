use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub mod models;
pub mod schema;

pub mod configs;
pub mod extensions;
pub mod graphql_resolvers;
pub mod graphql_types;
pub mod jwt_claims;
pub mod param_types;
pub mod responders;
pub mod rules;
pub mod tests;

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;
