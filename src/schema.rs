table! {
    todo (id) {
        id -> Int4,
        title -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
        todo_status_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    todo_status (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(todo -> todo_status (todo_status_id));
joinable!(todo -> users (user_id));

allow_tables_to_appear_in_same_query!(
    todo,
    todo_status,
    users,
);
