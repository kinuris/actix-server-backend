-- Your SQL goes here

CREATE TABLE users (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    username VARCHAR(20) NOT NULL,
    admin BOOLEAN NOT NULL DEFAULT FALSE,
    profile_img_link TEXT NOT NULL DEFAULT 'https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQwvGWjwjiCh8UCmLjeDGBj9iIZt7cyiynfwnYz_63_hg&s'
);