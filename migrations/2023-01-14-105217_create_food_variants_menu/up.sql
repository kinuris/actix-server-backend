-- Your SQL goes here

CREATE TABLE food_variants_menu (
    food_menu_id UUID NOT NULL,
    variant_name TEXT NOT NULL,
    price INT NOT NULL,
    stock INT NOT NULL,
    PRIMARY KEY (food_menu_id, variant_name),
    FOREIGN KEY (food_menu_id) REFERENCES food_menu (id)
)