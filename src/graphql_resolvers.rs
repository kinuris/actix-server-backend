use actix_session::Session;
use async_graphql::{Context, EmptySubscription, Object, Schema};
use diesel::{
    internal::table_macro::SelectStatement, query_builder::InsertStatement,
    NullableExpressionMethods, QueryDsl, RunQueryDsl,
};
use jsonwebtoken::{DecodingKey, Validation};

use crate::{
    extensions::Shared,
    graphql_types::{
        input_types::{FoodMenuInput, FoodMenuVariantInput, RegisterFoodAndVariants},
        output_types::{FoodAndVariantsOutput, FoodMenuOutput},
    },
    jwt_claims::AuthClaims,
    models::{FoodMenuItem, FoodMenuItemVariant, NewFoodMenuItem, NewFoodMenuItemVariant},
    schema::food_variants_menu,
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
                Ok(token) => serde_json::to_string(&token.claims).unwrap(),
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

    async fn get_registered_foods_with_variants(
        &self,
        ctx: &Context<'_>,
    ) -> Vec<FoodAndVariantsOutput> {
        let connection_pool = ctx.data::<ConnectionPool>().unwrap();

        use crate::schema::food_menu::dsl::*;
        use crate::schema::food_variants_menu::dsl::*;

        let result = food_menu.left_outer_join(food_variants_menu).select((
            (id, name, img_link, category),
            (food_menu_id, variant_name, price, stock).nullable(),
        )) as SelectStatement<_, _>;
        let result = result.load::<(FoodMenuItem, Option<FoodMenuItemVariant>)>(
            &mut connection_pool.get().unwrap(),
        );

        let result = result.map(|res| {
            res.into_iter()
                .map(|(food, variant)| (food, variant.unwrap_or_default()))
                .collect::<Vec<_>>()
        });

        let (mut foods, variants): (Vec<_>, Vec<_>) = result.unwrap().into_iter().unzip();
        let variant_groups = (1..=10)
            .map(|_| Vec::<FoodMenuItemVariant>::new())
            .collect::<Vec<_>>(); // Vec::<Vec<FoodMenuItemVariant>>::with_capacity(foods.len());

        foods.dedup();

        // SUGGESTION: Extract this
        let variant_groups =
            foods
                .iter()
                .enumerate()
                .fold(variant_groups, |mut acc, (index, food)| {
                    let mut variant_group = Vec::<FoodMenuItemVariant>::new();

                    variants.iter().for_each(|variant| {
                        if variant.food_menu_id == food.id {
                            variant_group.push(variant.clone());
                        }
                    });

                    acc[index] = variant_group;

                    acc
                });

        foods
            .into_iter()
            .zip(variant_groups.into_iter())
            .map(|x| x.into())
            .collect::<Vec<FoodAndVariantsOutput>>()
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

    async fn register_food_variant_with_id(
        &self,
        ctx: &Context<'_>,
        id: String,
        variant: FoodMenuVariantInput,
    ) -> String {
        let connection_pool = ctx.data::<ConnectionPool>().unwrap();
        let id = id.parse::<uuid::Uuid>();
        let id = match id {
            Ok(id) => id,
            Err(_) => return "Invalid ID".to_owned(),
        };

        let food_variant = NewFoodMenuItemVariant {
            food_menu_id: id,
            variant_name: &variant.variant_name,
            price: variant.price,
            stock: variant.stock,
        };

        let result = diesel::insert_into(food_variants_menu::table).values(&food_variant)
            as InsertStatement<_, _>;
        let result = result.get_result::<FoodMenuItemVariant>(&mut connection_pool.get().unwrap());
        match result {
            Ok(_) => {}
            Err(err) => return err.to_string(),
        };

        "Registered Variant".to_owned()
    }

    async fn register_food(&self, ctx: &Context<'_>, food: FoodMenuInput) -> String {
        let connection_pool = ctx.data::<ConnectionPool>().unwrap();

        let food_menu_item = NewFoodMenuItem {
            name: &food.name,
            category: &food.category,
            img_link: &food.img_link,
        };

        use crate::schema::food_menu;

        let result = diesel::insert_into(food_menu::table)
            .values(&food_menu_item)
            .get_result::<FoodMenuItem>(&mut connection_pool.get().unwrap())
            .expect("Error registering food");

        result.id.to_string()
    }

    async fn delete_foods_by_name(
        &self,
        ctx: &Context<'_>,
        food_names: Vec<String>,
    ) -> &'static str {
        let connection_pool = ctx.data::<ConnectionPool>().unwrap();

        use crate::schema::food_menu::dsl::*;
        use crate::schema::food_variants_menu::dsl::*;
        use diesel::expression_methods::ExpressionMethods;

        food_names.iter().for_each(|food_name| {
            let foods = food_menu
                .filter(name.eq(food_name))
                .get_results::<FoodMenuItem>(&mut connection_pool.get().unwrap())
                .unwrap();

            foods.iter().for_each(|food| {
                diesel::delete(food_variants_menu.filter(food_menu_id.eq(food.id)))
                    .execute(&mut connection_pool.get().unwrap())
                    .unwrap();
            });

            diesel::delete(food_menu.filter(name.eq(food_name)))
                .execute(&mut connection_pool.get().unwrap())
                .unwrap();
        });

        "Delete Foods By Name"
    }
}
