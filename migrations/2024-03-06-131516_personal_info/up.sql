-- Your SQL goes here
CREATE TABLE personal_info (
    id SERIAL PRIMARY KEY,
    full_name VARCHAR(255) NOT NULL,
    age INT NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(255) NOT NULL,
    address VARCHAR(1000) NOT NULL,
    profession VARCHAR(255) NOT NULL,
    birth_date VARCHAR(40) NOT NULL
);
