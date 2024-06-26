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

    #[sqlx::test]
    fn test_shared_lock() {
        let pool = setup_test_db().await.unwrap();

        let mut lock = DistributedLock::new(&pool, 1);
        lock.set_is_shared(true);
        let locked = lock.try_lock().await.unwrap();
        assert_eq!(locked, true)
    }

    #[sqlx::test]
    fn test_unlock() {
        let pool = setup_test_db().await.unwrap();

        let mut lock = DistributedLock::new(&pool, 1);
        lock.set_is_shared(true);
        lock.try_lock().await.unwrap();
        lock.unlock().await.unwrap();
    }

    #[sqlx::test]
    fn test_transaction_lock() {
        let pool = setup_test_db().await.unwrap();

        let lock = DistributedLock::new_with_attributes(&pool, 1, pglock_rust::LockType::TransactionLock, false);
        let locked = lock.try_lock().await.unwrap();
        assert_eq!(locked, true)
    }

    #[sqlx::test]
    fn test_transaction_shared_lock() {
        let pool = setup_test_db().await.unwrap();

        let lock = DistributedLock::new_with_attributes(&pool, 1, pglock_rust::LockType::TransactionLock, true);
        let locked = lock.try_lock().await.unwrap();
        assert_eq!(locked, true)
    }
}