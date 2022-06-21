use serde::Serialize;
use sqlx::PgPool;

#[derive(sqlx::FromRow, Serialize, Clone)]
pub struct Todo {
    pub id: i64,
    pub text: String
}

impl Todo {
    pub async fn get_all(pool: &PgPool) -> Vec<Todo> {
        sqlx::query_as::<_, Todo>("SELECT * FROM todos")
            .fetch_all(pool)
            .await.expect("Failed to load Todos from DB")
    }

    pub async fn create(&self, pool: &PgPool) {
        sqlx::query("INSERT INTO todos (text) VALUES ($1)")
            .bind(&self.text)
            .execute(pool)
            .await.expect("Couldn't create Todo in DB");
    }

    pub async fn update(&self, pool: &PgPool) {
        sqlx::query("UPDATE todos SET text = $1 WHERE id = $2")
            .bind(&self.text)
            .bind(self.id)
            .execute(pool)
            .await.expect("Couldn't create Todo in DB");
    }

    pub async fn delete(&self, pool: &PgPool) {
        sqlx::query("DELETE FROM todos WHERE id = $1")
            .bind(self.id)
            .execute(pool)
            .await.expect("Couldn't create Todo in DB");
    }
}
