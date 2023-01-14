// @generated automatically by Diesel CLI.

diesel::table! {
    food_menu (id) {
        id -> Uuid,
        name -> Text,
        img_link -> Text,
        category -> Text,
    }
}

diesel::table! {
    food_variants_menu (food_menu_id, variant_name) {
        food_menu_id -> Uuid,
        variant_name -> Text,
        price -> Int4,
        stock -> Int4,
    }
}

diesel::joinable!(food_variants_menu -> food_menu (food_menu_id));

diesel::allow_tables_to_appear_in_same_query!(food_menu, food_variants_menu,);
