use diesel::prelude::*;
use crate::models::{NewTask, NewTaskStatus, NewUser, NewUserTask, Task, TaskStatus, User, UserTask};
use crate::schema::{users, tasks, user_tasks, task_statuses};


// pub trait CrudOperations<T1, T2, T3, T4>
pub trait CrudOperations<Conn, Id, NewEntity, Entity> {
    fn create(conn: &mut Conn, new_entity: NewEntity) -> anyhow::Result<Entity>;
    fn read(conn: &mut Conn, id: Id) -> anyhow::Result<Option<Entity>>;
    fn update(conn: &mut Conn, id: Id, updated_entity: NewEntity) -> anyhow::Result<Entity>;
    fn delete(conn: &mut Conn, id: Id) -> anyhow::Result<usize>;
    fn read_all(conn: &mut Conn) -> anyhow::Result<Vec<Entity>>;
}

impl<'a> CrudOperations<SqliteConnection, i32, NewUser<'a>, User> for User {
    fn create(conn: &mut SqliteConnection, new_user: NewUser<'a>) -> anyhow::Result<User> {
        let user = diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(conn)?;
        Ok(user)
    }

    fn read(conn: &mut SqliteConnection, id: i32) -> anyhow::Result<Option<User>> {
        let user = users::table.find(id).first(conn).optional()?;
        Ok(user)
    }

    fn update(conn: &mut SqliteConnection, id: i32, updated_user: NewUser<'a>) -> anyhow::Result<User> {
        diesel::update(users::table.find(id))
            .set((users::name.eq(updated_user.name), users::email.eq(updated_user.email), users::active.eq(updated_user.active)))
            .execute(conn)?;
        let user = users::table.find(id).first(conn)?;
        Ok(user)
    }

    fn delete(conn: &mut SqliteConnection, id: i32) -> anyhow::Result<usize> {
        let count = diesel::delete(users::table.find(id)).execute(conn)?;
        Ok(count)
    }

    fn read_all(conn: &mut SqliteConnection) -> anyhow::Result<Vec<User>> {
        let results = users::table.load::<User>(conn)?;
        Ok(results)
    }
}


impl<'a> CrudOperations<SqliteConnection, i32, NewTask<'a>, Task> for Task {
    fn create(conn: &mut SqliteConnection, new_task: NewTask<'a>) -> anyhow::Result<Task> {
        let task = diesel::insert_into(tasks::table)
            .values(&new_task)
            .returning(Task::as_returning())
            .get_result(conn)?;
        Ok(task)
    }

    fn read(conn: &mut SqliteConnection, id: i32) -> anyhow::Result<Option<Task>> {
        let tasks = tasks::table.find(id).first(conn).optional()?;
        Ok(tasks)
    }

    fn update(conn: &mut SqliteConnection, id: i32, updated_task: NewTask<'a>) -> anyhow::Result<Task> {
        diesel::update(tasks::table.find(id))
            .set(tasks::task_name.eq(updated_task.task_name))
            .execute(conn)?;
        let task = tasks::table.find(id).first(conn)?;
        Ok(task)
    }

    fn delete(conn: &mut SqliteConnection, id: i32) -> anyhow::Result<usize> {
        let count = diesel::delete(tasks::table.find(id)).execute(conn)?;
        Ok(count)
    }

    fn read_all(conn: &mut SqliteConnection) -> anyhow::Result<Vec<Task>> {
        let results = tasks::table.load::<Task>(conn)?;
        Ok(results)
    }
}


impl<'a> CrudOperations<SqliteConnection, i32, NewTaskStatus<'a>, TaskStatus> for TaskStatus {
    fn create(conn: &mut SqliteConnection, new_task_status: NewTaskStatus<'a>) -> anyhow::Result<TaskStatus> {
        let task_status = diesel::insert_into(task_statuses::table)
            .values(&new_task_status)
            .returning(TaskStatus::as_returning())
            .get_result(conn)?;
        Ok(task_status)
    }

    fn read(conn: &mut SqliteConnection, id: i32) -> anyhow::Result<Option<TaskStatus>> {
        let task_status = task_statuses::table.find(id).first(conn).optional()?;
        Ok(task_status)
    }

    fn update(conn: &mut SqliteConnection, id: i32, updated_task_status: NewTaskStatus<'a>) -> anyhow::Result<TaskStatus> {
        diesel::update(task_statuses::table.find(id))
            .set(task_statuses::status_name.eq(updated_task_status.status_name))
            .execute(conn)?;
        let task_status = task_statuses::table.find(id).first(conn)?;
        Ok(task_status)
    }

    fn delete(conn: &mut SqliteConnection, id: i32) -> anyhow::Result<usize> {
        let count = diesel::delete(task_statuses::table.find(id)).execute(conn)?;
        Ok(count)
    }

    fn read_all(conn: &mut SqliteConnection) -> anyhow::Result<Vec<TaskStatus>> {
        let results = task_statuses::table.load::<TaskStatus>(conn)?;
        Ok(results)
    }
}


impl CrudOperations<SqliteConnection, (i32, i32), NewUserTask, UserTask> for UserTask {
    fn create(conn: &mut SqliteConnection, new: NewUserTask) -> anyhow::Result<UserTask> {
        use crate::schema::user_tasks;

        diesel::insert_into(user_tasks::table)
            .values(&new)
            .execute(conn)?;

        Self::read(conn, (new.user_id, new.task_id))?
            .ok_or_else(|| anyhow::anyhow!("Failed to retrieve inserted UserTask"))
    }

    fn read(conn: &mut SqliteConnection, id: (i32, i32)) -> anyhow::Result<Option<UserTask>> {
        use crate::schema::user_tasks::dsl::*;

        let user_task = user_tasks
            .filter(user_id.eq(id.0))
            .filter(task_id.eq(id.1))
            .first::<UserTask>(conn)
            .optional()?;

        Ok(user_task)
    }

    fn update(conn: &mut SqliteConnection, id: (i32, i32), updated_user_task: NewUserTask) -> anyhow::Result<UserTask> {
        diesel::update(user_tasks::table
            .filter(user_tasks::user_id.eq(id.0))
            .filter(user_tasks::task_id.eq(id.1)))
            .set(user_tasks::task_status_id.eq(updated_user_task.task_status_id))
            .execute(conn)?;
        let user_task = user_tasks::table
            .filter(user_tasks::user_id.eq(id.0))
            .filter(user_tasks::task_id.eq(id.1))
            .first(conn)?;
        Ok(user_task)
    }

    fn delete(conn: &mut SqliteConnection, id: (i32, i32)) -> anyhow::Result<usize> {
        let count = diesel::delete(user_tasks::table
            .filter(user_tasks::user_id.eq(id.0))
            .filter(user_tasks::task_id.eq(id.1)))
            .execute(conn)?;
        Ok(count)
    }

    fn read_all(conn: &mut SqliteConnection) -> anyhow::Result<Vec<UserTask>> {
        let results = user_tasks::table.load::<UserTask>(conn)?;
        Ok(results)
    }
}

