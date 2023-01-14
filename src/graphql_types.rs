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
        pub name: String,
        pub price: String,
        pub stock: u32,
    }

    #[derive(Debug, InputObject)]
    pub struct RegisterFoodAndVariants {
        pub food: FoodMenuInput,
        pub variants: Vec<FoodMenuVariantInput>,
    }
}

pub mod output_types {
    use async_graphql::{ComplexObject, SimpleObject};

    #[derive(SimpleObject)]
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
}
