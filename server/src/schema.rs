table! {
    day (id) {
        id -> Uuid,
        year -> Int2,
        month -> Int2,
        day_of_month -> Int2,
    }
}

table! {
    dayentry (id) {
        id -> Uuid,
        day -> Uuid,
        name -> Text,
        value -> Float8,
    }
}

joinable!(dayentry -> day (day));

allow_tables_to_appear_in_same_query!(
    day,
    dayentry,
);
