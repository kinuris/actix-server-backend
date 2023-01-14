use actix_session::Session;
use async_graphql::{Context, EmptySubscription, Object, Schema};
use diesel::{internal::table_macro::SelectStatement, QueryDsl, RunQueryDsl};
use jsonwebtoken::{DecodingKey, Validation};

use crate::{
    extensions::Shared,
    graphql_types::{
        input_types::{FoodMenuInput, RegisterFoodAndVariants},
        output_types::FoodMenuOutput,
    },
    jwt_claims::AuthClaims,
    models::{FoodMenuItem, NewFoodMenuItem},
    ConnectionPool,
};

pub type AppGraphQLSchema = Schema<Query, Mutation, EmptySubscription>;
pub struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> &'static str {
        "Hello from server"
    }

    async fn get_token_status(&self, ctx: &Context<'_>) -> String {
        let session = ctx.data::<Shared<Session>>().unwrap();
        let result = session.get::<String>("auth_token").unwrap();
        let secret = dotenv::var("SECRET").unwrap();

        if result.is_none() {
            "You don't have a token".to_owned()
        } else {
            let auth_token = jsonwebtoken::decode::<AuthClaims>(
                result.as_ref().unwrap(),
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::default(),
            );
            
            match auth_token {
                Ok(token) => {
                    serde_json::to_string(&token.claims).unwrap()
                },
                Err(_) => {
                    session.remove("auth_token").unwrap();
                    "Token is no longer valid".to_owned()
                }
            }
        }
    }

    async fn get_registered_foods(&self, ctx: &Context<'_>) -> Vec<FoodMenuOutput> {
        let connection_pool = ctx.data::<ConnectionPool>().unwrap();

        use crate::schema::food_menu::dsl::*;

        let result = food_menu.select((id, name, category, img_link)) as SelectStatement<_, _>;
        let result = result.load::<FoodMenuItem>(&mut connection_pool.get().unwrap());

        result
            .unwrap()
            .into_iter()
            .map(|item| item.into_output_type())
            .collect()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn register_food_and_variants(
        &self,
        ctx: &Context<'_>,
        food_and_variants: RegisterFoodAndVariants,
    ) -> &'static str {
        "Hello"
    }

    async fn register_food(&self, ctx: &Context<'_>, food: FoodMenuInput) -> &'static str {
        let connection_pool = ctx.data::<ConnectionPool>().unwrap();

        let food_menu_item = NewFoodMenuItem {
            name: &food.name,
            category: &food.category,
            img_link: &food.img_link,
        };

        use crate::schema::food_menu;

        diesel::insert_into(food_menu::table)
            .values(&food_menu_item)
            .execute(&mut connection_pool.get().unwrap())
            .expect("Error registering food");

        "Register Food"
    }

    async fn delete_foods(&self, ctx: &Context<'_>, foods: Vec<FoodMenuInput>) -> &'static str {
        "Deleted Foods"
    }

    async fn delete_foods_by_name(
        &self,
        ctx: &Context<'_>,
        food_names: Vec<String>,
    ) -> &'static str {
        "Delete Foods By Name"
    }
}
