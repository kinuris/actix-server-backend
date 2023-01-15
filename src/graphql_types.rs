pub mod input_types {
    use async_graphql::InputObject;

    #[derive(Debug, InputObject)]
    pub struct FoodMenuInput {
        pub name: String,
        pub img_link: String,
        pub category: String,
    }

    #[derive(Debug, InputObject)]
    pub struct FoodMenuVariantInput {
        pub variant_name: String,
        pub price: i32,
        pub stock: i32,
    }

    #[derive(Debug, InputObject)]
    pub struct RegisterFoodAndVariants {
        pub food: FoodMenuInput,
        pub variants: Vec<FoodMenuVariantInput>,
    }
}

pub mod output_types {
    use async_graphql::{ComplexObject, SimpleObject};

    use crate::models::{FoodMenuItem, FoodMenuItemVariant};

    #[derive(SimpleObject, Debug)]
    #[graphql(complex)]
    pub struct FoodMenuOutput {
        #[graphql(skip)]
        pub id: uuid::Uuid,

        pub name: String,
        pub img_link: String,
        pub category: String,
    }

    #[ComplexObject]
    impl FoodMenuOutput {
        async fn id(&self) -> String {
            self.id.to_string()
        }
    }

    #[derive(SimpleObject, Debug)]
    pub struct FoodMenuVariantOutput {
        pub variant_name: String,
        pub price: i32,
        pub stock: i32,
    }

    #[derive(SimpleObject, Debug)]
    pub struct FoodAndVariantsOutput {
        food: FoodMenuOutput,
        variants: Vec<FoodMenuVariantOutput>,
    }

    impl From<(FoodMenuItem, Vec<FoodMenuItemVariant>)> for FoodAndVariantsOutput {
        fn from(value: (FoodMenuItem, Vec<FoodMenuItemVariant>)) -> Self {
            FoodAndVariantsOutput {
                food: value.0.into_output_type(),
                variants: value.1.into_iter().map(|x| x.into_output_type()).collect(),
            }
        }
    }
}
