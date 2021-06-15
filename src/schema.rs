table! {
    authentication_entries (id) {
        id -> Uuid,
        account_name -> Varchar,
        secret -> Uuid,
        policy -> Jsonb,
        policy_base64 -> Text,
        valid_until -> Timestamp,
        blockchain_index -> Nullable<Int8>,
    }
}
