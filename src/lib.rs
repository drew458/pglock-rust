use sqlx::PgPool;

#[derive(Debug, Clone, Copy)]
pub struct DistributedLock {
    key: i64,
    lock_type: LockType,
    is_shared: bool
}

#[derive(Debug, Clone, Copy)]
pub enum LockType {
    SessionLock,
    TransactionLock
}

impl DistributedLock {

    pub fn new(key: i64) -> DistributedLock {

        DistributedLock {
            key,
            lock_type: LockType::SessionLock,
            is_shared: false
        }
    }

    pub fn new_with_attributes(key: i64, lock_type: LockType, is_shared: bool) -> DistributedLock {

        DistributedLock {
            key,
            lock_type,
            is_shared
        }
    }

    pub fn key(&self) -> i64 {
        self.key
    }

    pub fn lock_type(&self) -> LockType {
        self.lock_type
    }

    pub fn is_shared(&self) -> bool {
        self.is_shared
    }

    pub fn set_key(&mut self, key: i64) {
        self.key = key
    }

    pub fn set_lock_type(&mut self, lock_type: LockType) {
        self.lock_type = lock_type
    }

    pub fn set_is_shared(&mut self, is_shared: bool) {
        self.is_shared = is_shared
    }
}

pub async fn lock(pool: &PgPool, lock: &DistributedLock) -> Result<(), sqlx::Error> {

    sqlx::query(&build_query(lock.is_shared(), true))
        .bind(lock.key())
        .execute(pool).await?;

    Ok(())
}

pub async fn try_lock(pool: &PgPool, lock: &DistributedLock) -> Result<bool, sqlx::Error> {
    let ret: (bool, ) = sqlx::query_as(&build_query(lock.is_shared(), false))
        .bind(lock.key())
        .fetch_one(pool).await?;

    Ok(ret.0)
}

pub async fn unlock(pool: &PgPool, lock: &DistributedLock) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT pg_catalog.pg_advisory_unlock($1)")
        .bind(lock.key())
        .execute(pool).await?;

    Ok(())
}

fn build_query(is_shared: bool, is_wait: bool) -> String {

    let lock_type = if is_wait { "advisory_lock" } else { "try_advisory_lock" };
    let shared_part = if is_shared { "_shared" } else { "" };
    
    format!("SELECT pg_catalog.pg_{}{}($1)", lock_type, shared_part)
}