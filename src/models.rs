use diesel::prelude::*;

#[derive(Queryable, Debug, PartialEq)]
pub struct FoodMenuItem {
    pub id: uuid::Uuid,
    pub name: String,
    pub img_link: String,
    pub category: String,
}

impl FoodMenuItem {
    pub fn owned_output_type(&self) -> FoodMenuOutput {
        FoodMenuOutput {
            id: self.id,
            name: self.name.clone(),
            img_link: self.img_link.clone(),
            category: self.category.clone(),
        }
    }

    pub fn into_output_type(self) -> FoodMenuOutput {
        FoodMenuOutput {
            id: self.id,
            name: self.name,
            img_link: self.img_link,
            category: self.category,
        }
    }

    pub fn eq_id(&self, other: &FoodMenuItem) -> bool {
        self.id == other.id
    }
}

#[derive(Queryable, Debug, PartialEq, Clone)]
pub struct FoodMenuItemVariant {
    pub food_menu_id: uuid::Uuid,
    pub variant_name: String,
    pub price: i32,
    pub stock: i32,
}

impl FoodMenuItemVariant {
    pub fn into_output_type(self) -> FoodMenuVariantOutput {
        FoodMenuVariantOutput {
            variant_name: self.variant_name,
            price: self.price,
            stock: self.stock,
        }
    }

    pub fn eq_id(&self, other: &FoodMenuItemVariant) -> bool {
        self.food_menu_id == other.food_menu_id
    }
}

impl Default for FoodMenuItemVariant {
    fn default() -> Self {
        Self {
            food_menu_id: Default::default(),
            variant_name: Default::default(),
            price: Default::default(),
            stock: Default::default(),
        }
    }
}

use crate::{
    graphql_types::output_types::{FoodMenuOutput, FoodMenuVariantOutput},
    schema::{food_menu, food_variants_menu},
};

#[derive(Debug, Insertable)]
#[diesel(table_name = food_menu)]
pub struct NewFoodMenuItem<'a> {
    pub name: &'a str,
    pub img_link: &'a str,
    pub category: &'a str,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = food_variants_menu)]
pub struct NewFoodMenuItemVariant<'a> {
    pub food_menu_id: uuid::Uuid,
    pub variant_name: &'a str,
    pub price: i32,
    pub stock: i32,
}

#[derive(Debug, Queryable)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub password: String,
    pub username: String,
    pub admin: bool,
    pub profile_img_link: String,
}

use crate::schema::users;

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
    pub username: &'a str,
    pub profile_img_link: Option<&'a str>,
}
