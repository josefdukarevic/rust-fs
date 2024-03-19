// @generated automatically by Diesel CLI.

diesel::table! {
    personal_info (id) {
        id -> Int4,
        #[max_length = 255]
        full_name -> Varchar,
        age -> Int4,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        phone -> Varchar,
        #[max_length = 1000]
        address -> Varchar,
        #[max_length = 255]
        profession -> Varchar,
        birth_date -> Date,
    }
}
