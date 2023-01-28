pub mod input_types {
    use async_graphql::InputObject;

    use crate::models::NewFoodMenuItemVariant;

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

    impl FoodMenuVariantInput {
        pub fn as_insertable(&self, id: uuid::Uuid) -> NewFoodMenuItemVariant {
            NewFoodMenuItemVariant {
                food_menu_id: id,
                variant_name: &self.variant_name,
                price: self.price,
                stock: self.stock,
            }
        }
    }

    #[derive(Debug, InputObject)]
    pub struct RegisterFoodAndVariants {
        pub food: FoodMenuInput,
        pub variants: Vec<FoodMenuVariantInput>,
    }
}

pub mod output_types {
    use async_graphql::{ComplexObject, Enum, SimpleObject, Union};

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

    impl From<FoodMenuItem> for FoodMenuOutput {
        fn from(
            FoodMenuItem {
                id,
                category,
                img_link,
                name,
            }: FoodMenuItem,
        ) -> Self {
            Self {
                id,
                category,
                img_link,
                name,
            }
        }
    }

    #[derive(SimpleObject, Debug)]
    pub struct FoodMenuVariantOutput {
        pub variant_name: String,
        pub price: i32,
        pub stock: i32,
    }

    impl From<FoodMenuItemVariant> for FoodMenuVariantOutput {
        fn from(value: FoodMenuItemVariant) -> Self {
            Self {
                variant_name: value.variant_name,
                price: value.price,
                stock: value.stock,
            }
        }
    }

    #[derive(SimpleObject, Debug)]
    pub struct FoodAndVariantsOutput {
        food: FoodMenuOutput,
        variants: Vec<FoodMenuVariantOutput>,
    }

    impl From<(FoodMenuItem, Vec<FoodMenuItemVariant>)> for FoodAndVariantsOutput {
        fn from(value: (FoodMenuItem, Vec<FoodMenuItemVariant>)) -> Self {
            FoodAndVariantsOutput {
                food: value.0.into(),
                variants: value.1.into_iter().map(|x| x.into()).collect(),
            }
        }
    }

    #[derive(Union)]
    pub enum RegisterFoodStatus {
        Status(RegisterFoodStateWrapper),
        Message(StringWrapper),
    }

    // SUGGESTION: Put wrappers in their own module
    #[derive(SimpleObject)]
    pub struct RegisterFoodStateWrapper {
        pub state: RegisterFoodState,
    }

    impl From<RegisterFoodState> for RegisterFoodStateWrapper {
        fn from(value: RegisterFoodState) -> Self {
            RegisterFoodStateWrapper { state: value }
        }
    }

    #[derive(SimpleObject)]
    pub struct StringWrapper {
        pub state: String,
    }

    impl From<String> for StringWrapper {
        fn from(value: String) -> Self {
            StringWrapper { state: value }
        }
    }

    #[derive(Enum, Copy, Clone, PartialEq, Eq, Debug)]
    pub enum RegisterFoodState {
        MustBeAdmin,
    }

    #[derive(Enum, Copy, Clone, PartialEq, Eq, Debug)]
    pub enum SignupStatus {
        Success,
        EmailNotAvailable,
        UsernameNotAvailable,
        InvalidPasswordCallingTheFBI,
        SessionStillValid,
    }

    #[derive(Enum, Copy, Clone, PartialEq, Eq)]
    pub enum LoginStatus {
        Success,
        WrongEmailOrPassword,
        SessionStillValid,
    }

    #[derive(Enum, Copy, Clone, PartialEq, Eq)]
    pub enum DeleteFoodStatus {
        Success,
        FoodDoesNotExist,
        MustBeAdmin,
    }

    #[derive(Enum, Copy, Clone, PartialEq, Eq)]
    pub enum RegisterFoodVariantWithId {
        Success,
        MustBeAdmin,
        FoodDoesNotExist,
        InvalidId,
        VariantAlreadyExists,
        UnknownError,
    }
}
