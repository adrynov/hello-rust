#[allow(unused_imports)]
#[allow(dead_code)]
use crate::commands::{connect::connect, error::AppError};
use crate::models::task::Task;

pub async fn add_task(task: &Task) -> Result<(), Box<dyn std::error::Error>> {
    let title = task.title.clone();
    let description = task.description.clone();
    // check data
    if title.is_empty() || title.len() > 255 {
        println!("Title must be between 1 and 255 characters");

        return Err(Box::new(AppError::ValidationError(
            "Title must be between 1 and 255 characters".to_string(),
        )));
    }

    let pool = connect().await?;

    sqlx::query("INSERT INTO tasks (title, description) VALUES ($1, $2)")
        .bind(&title)
        .bind(&description)
        .execute(&pool)
        .await
        .map_err(|e| Box::new(AppError::DatabaseError(e.to_string())))?;

    println!("Task '{:?}' added!", task);
    Ok(())
}
