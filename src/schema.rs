table! {
    users (id) {
        id -> Int8,
        public_id -> Uuid,
        name -> Varchar,
        create_date -> Timestamptz,
        update_date -> Timestamptz,
    }
}
