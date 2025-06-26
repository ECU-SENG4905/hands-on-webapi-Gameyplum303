// @generated automatically by Diesel CLI.

diesel::table! {
    task_statuses (task_status_id) {
        task_status_id -> Integer,
        status_name -> Text,
    }
}

diesel::table! {
    tasks (task_id) {
        task_id -> Integer,
        task_name -> Text,
    }
}

diesel::table! {
    user_tasks (user_id, task_id) {
        user_id -> Integer,
        task_id -> Integer,
        task_status_id -> Integer,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        name -> Text,
        email -> Text,
        active -> Bool,
    }
}

diesel::joinable!(user_tasks -> task_statuses (task_status_id));
diesel::joinable!(user_tasks -> tasks (task_id));
diesel::joinable!(user_tasks -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(task_statuses, tasks, user_tasks, users,);
