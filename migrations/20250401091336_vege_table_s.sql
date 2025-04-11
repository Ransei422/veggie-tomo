-- Table for storing registred vegetables

CREATE TABLE vegetables (
    id SERIAL PRIMARY KEY,
    vegetable_name TEXT NOT NULL UNIQUE,
    vegetable_family TEXT NOT NULL,
    vegetable_species TEXT NOT NULL
);