use std::time::SystemTime;

use actix_session::Session;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use async_graphql::{Context, EmptySubscription, Object, Schema};
use diesel::{
    internal::table_macro::SelectStatement, query_builder::InsertStatement,
    NullableExpressionMethods, QueryDsl, RunQueryDsl,
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};

use crate::{
    extensions::Shared,
    graphql_types::{
        input_types::{FoodMenuInput, FoodMenuVariantInput, RegisterFoodAndVariants},
        output_types::{
            DeleteFoodStatus, FoodAndVariantsOutput, FoodMenuOutput, LoginStatus,
            RegisterFoodState, RegisterFoodStatus, RegisterFoodVariantWithId, SignupStatus,
        },
    },
    jwt_claims::AuthClaims,
    models::{
        FoodMenuItem, FoodMenuItemVariant, NewFoodMenuItem, NewFoodMenuItemVariant, NewUser, User,
    },
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
        use diesel::expression_methods::ExpressionMethods;

        let result = (food_menu.select((id, name, img_link, category)) as SelectStatement<_, _>)
            .order(name.asc());
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
        use diesel::expression_methods::ExpressionMethods;

        let result = (food_menu.left_outer_join(food_variants_menu).select((
            (id, name, img_link, category),
            (food_menu_id, variant_name, price, stock).nullable(),
        )) as SelectStatement<_, _>)
            .order((name.asc(), price.asc()));
        let result = result.load::<(FoodMenuItem, Option<FoodMenuItemVariant>)>(
            &mut connection_pool.get().unwrap(),
        );

        let result = result.map(|res| {
            res.into_iter()
                .map(|(food, variant)| (food, variant.unwrap_or_default()))
                .collect::<Vec<_>>()
        });

        let (mut foods, variants): (Vec<_>, Vec<_>) = result.unwrap().into_iter().unzip();
        let variant_groups = (0..foods.len())
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

    async fn logout(&self, ctx: &Context<'_>) -> &'static str {
        let session = ctx.data::<Shared<Session>>().unwrap();

        session.remove("auth_token");

        "Logged Out"
    }

    async fn login(&self, ctx: &Context<'_>, email: String, password: String) -> LoginStatus {
        let session = ctx.data::<Shared<Session>>().unwrap();
        let auth_token = session.get::<String>("auth_token").unwrap();
        let secret = dotenv::var("SECRET").unwrap();

        if auth_token.is_some() {
            let result = jsonwebtoken::decode::<AuthClaims>(
                auth_token.as_ref().unwrap(),
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::default(),
            );

            match result {
                Ok(_) => return LoginStatus::SessionStillValid,
                Err(_) => session.remove("auth_token").unwrap(),
            };
        }

        let connection_pool = ctx.data::<ConnectionPool>().unwrap();

        use crate::schema::users;
        use diesel::expression_methods::ExpressionMethods;

        let user = users::table
            .filter(users::email.eq(&email))
            .get_result::<User>(&mut connection_pool.get().unwrap());
        let user = match user {
            Ok(user) => user,
            Err(_) => return LoginStatus::WrongEmailOrPassword,
        };

        // SUGGESTION: Implement spawn_blocking or equivalent thereof
        let hash = PasswordHash::new(&user.password).unwrap();
        let result = Argon2::default().verify_password(password.as_bytes(), &hash);

        if result.is_err() {
            return LoginStatus::WrongEmailOrPassword;
        }

        let claim = AuthClaims {
            admin: user.admin,
            id: user.id.to_string(),
            exp: SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize
                + 3600,
        };

        let token = jsonwebtoken::encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(dotenv::var("SECRET").unwrap().as_ref()),
        );

        session.insert("auth_token", token.unwrap()).unwrap();

        LoginStatus::Success
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn register_food_and_variants(
        &self,
        ctx: &Context<'_>,
        food_and_variants: RegisterFoodAndVariants,
    ) -> String {
        let connection_pool = ctx.data::<ConnectionPool>().unwrap();
        let RegisterFoodAndVariants { food, variants } = food_and_variants;

        let id = match self.register_food(ctx, food).await {
            Ok(res) => match res {
                RegisterFoodStatus::Status(status) => return format!("{:?}", status.state),
                RegisterFoodStatus::Message(id) => id.state.parse::<uuid::Uuid>().unwrap(),
            },
            Err(err) => return err.message,
        };

        use crate::schema::food_variants_menu;

        let variants = variants
            .iter()
            .map(|variant| variant.as_insertable(id))
            .collect::<Vec<_>>();

        let result = diesel::insert_into(food_variants_menu::table)
            .values(&variants)
            .execute(&mut connection_pool.get().unwrap());

        if result.is_ok() {
            "Success".to_string()
        } else {
            result.err().unwrap().to_string()
        }
    }

    async fn register_food_variant_with_id(
        &self,
        ctx: &Context<'_>,
        id: String,
        variant: FoodMenuVariantInput,
    ) -> RegisterFoodVariantWithId {
        let session = ctx.data::<Shared<Session>>().unwrap();
        let auth_token = session.get::<String>("auth_token").unwrap();
        let secret = dotenv::var("SECRET").unwrap();

        if auth_token.is_none() {
            return RegisterFoodVariantWithId::MustBeAdmin;
        }

        let result = jsonwebtoken::decode::<AuthClaims>(
            auth_token.as_ref().unwrap(),
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        );

        match result {
            Ok(token) => {
                if !token.claims.admin {
                    return RegisterFoodVariantWithId::MustBeAdmin;
                }
            }
            Err(_) => {
                session.remove("auth_token").unwrap();

                return RegisterFoodVariantWithId::MustBeAdmin;
            }
        };

        let connection_pool = ctx.data::<ConnectionPool>().unwrap();
        let id = id.parse::<uuid::Uuid>();
        let id = match id {
            Ok(id) => id,
            Err(_) => return RegisterFoodVariantWithId::InvalidId,
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
            Err(err) => match err {
                diesel::result::Error::DatabaseError(kind, _) => match kind {
                    diesel::result::DatabaseErrorKind::UniqueViolation => {
                        return RegisterFoodVariantWithId::VariantAlreadyExists
                    }
                    diesel::result::DatabaseErrorKind::ForeignKeyViolation => {
                        return RegisterFoodVariantWithId::FoodDoesNotExist
                    }
                    _ => return RegisterFoodVariantWithId::UnknownError,
                },
                _ => return RegisterFoodVariantWithId::UnknownError,
            },
        };

        RegisterFoodVariantWithId::Success
    }

    async fn register_food(&self, ctx: &Context<'_>, food: FoodMenuInput) -> RegisterFoodStatus {
        let session = ctx.data::<Shared<Session>>().unwrap();
        let auth_token = session.get::<String>("auth_token").unwrap();
        let secret = dotenv::var("SECRET").unwrap();

        if auth_token.is_none() {
            return RegisterFoodStatus::Status(RegisterFoodState::MustBeAdmin.into());
        }

        let result = jsonwebtoken::decode::<AuthClaims>(
            auth_token.as_ref().unwrap(),
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        );

        match result {
            Ok(token) => {
                if !token.claims.admin {
                    return RegisterFoodStatus::Status(RegisterFoodState::MustBeAdmin.into());
                }
            }
            Err(_) => {
                session.remove("auth_token").unwrap();

                return RegisterFoodStatus::Status(RegisterFoodState::MustBeAdmin.into());
            }
        };

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

        RegisterFoodStatus::Message(result.id.to_string().into())
    }

    async fn delete_foods_by_name(
        &self,
        ctx: &Context<'_>,
        food_names: Vec<String>,
    ) -> DeleteFoodStatus {
        let session = ctx.data::<Shared<Session>>().unwrap();
        let auth_token = session.get::<String>("auth_token").unwrap();
        let secret = dotenv::var("SECRET").unwrap();

        if auth_token.is_none() {
            return DeleteFoodStatus::MustBeAdmin;
        }

        let result = jsonwebtoken::decode::<AuthClaims>(
            auth_token.as_ref().unwrap(),
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        );

        match result {
            Ok(token) => {
                if !token.claims.admin {
                    return DeleteFoodStatus::MustBeAdmin;
                }
            }
            Err(_) => {
                session.remove("auth_token").unwrap();

                return DeleteFoodStatus::MustBeAdmin;
            }
        };

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

        DeleteFoodStatus::Success
    }

    async fn signup(
        &self,
        ctx: &Context<'_>,
        username: String,
        email: String,
        password: String,
        profile_img_link: String,
    ) -> SignupStatus {
        let session = ctx.data::<Shared<Session>>().unwrap();
        let auth_token = session.get::<String>("auth_token").unwrap();
        let secret = dotenv::var("SECRET").unwrap();

        if auth_token.is_some() {
            let result = jsonwebtoken::decode::<AuthClaims>(
                auth_token.as_ref().unwrap(),
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::default(),
            );

            match result {
                Ok(_) => return SignupStatus::SessionStillValid,
                Err(_) => session.remove("auth_token").unwrap(),
            };
        }

        let connection_pool = ctx.data::<ConnectionPool>().unwrap();

        use crate::schema::users;
        use diesel::expression_methods::*;

        let possible_user = users::table
            .filter(users::username.eq(&username).or(users::email.eq(&email)))
            .get_result::<User>(&mut connection_pool.get().unwrap());

        match possible_user {
            Ok(user) => {
                if user.email == email {
                    return SignupStatus::EmailNotAvailable;
                } else if user.username == username {
                    return SignupStatus::UsernameNotAvailable;
                }
            }
            Err(_) => {}
        }

        // SUGGESTION: Implement spawn_blocking or equivalent thereof
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password = argon2.hash_password(password.as_bytes(), &salt);

        let password = match password {
            Ok(pass) => pass.to_string(),
            Err(_) => return SignupStatus::InvalidPasswordCallingTheFBI,
        };

        let new_user = NewUser {
            email: &email,
            password: &password,
            profile_img_link: &profile_img_link,
            username: &username,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&mut connection_pool.get().unwrap())
            .unwrap();

        SignupStatus::Success
    }
}
