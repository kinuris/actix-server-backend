use async_graphql::{Context, EmptySubscription, Object, Schema};
use diesel::{internal::table_macro::SelectStatement, QueryDsl, RunQueryDsl};

use crate::{
    graphql_types::{
        input_types::{FoodMenuInput, RegisterFoodAndVariants},
        output_types::FoodMenuOutput,
    },
    models::{FoodMenuItem, NewFoodMenuItem},
    ConnectionPool,
};

pub type AppGraphQLSchema = Schema<Query, Mutation, EmptySubscription>;
pub struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> &'static str {
        "Hello There"
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

    async fn delete_foods_by_name(&self, ctx: &Context<'_>, food_names: Vec<String>) -> &'static str {
        "Delete Foods By Name"
    }
}
