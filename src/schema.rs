table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Varchar,
        published -> Tinyint,
    }
}

table! {
    sys_config (variable) {
        variable -> Varchar,
        value -> Nullable<Varchar>,
        set_time -> Nullable<Timestamp>,
        set_by -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    sys_config,
    users,
);
