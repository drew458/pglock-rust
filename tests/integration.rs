use sqlx::PgPool;

#[cfg(test)]
mod tests {
    use pglock_rust::DistributedLock;
    use sqlx::Error;
    use super::*;

    async fn setup_test_db() -> Result<PgPool, Error> {

        let pool = PgPool::connect("postgresql://postgres@localhost:5432/postgres").await.expect("Failed to create pool");
        
        Ok(pool)
    }

    #[sqlx::test]
    fn test_lock() {
        let pool = setup_test_db().await.unwrap();

        let lock = DistributedLock::new(&pool, 1);
        let locked = lock.try_lock().await.unwrap();
        assert_eq!(locked, true)
    }
}