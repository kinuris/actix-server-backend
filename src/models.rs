use diesel::prelude::*;

#[derive(Queryable, Debug)]
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
}

use crate::{graphql_types::output_types::FoodMenuOutput, schema::food_menu};

#[derive(Debug, Insertable)]
#[diesel(table_name = food_menu)]
pub struct NewFoodMenuItem<'a> {
    pub name: &'a str,
    pub img_link: &'a str,
    pub category: &'a str,
}
