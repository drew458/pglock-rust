use sqlx::PgPool;

pub struct DistributedLock<'a> {
    pool: &'a PgPool,
    key: i64,
    lock_type: LockType,
    is_shared: bool
}

pub enum LockType {
    SessionLock,
    TransactionLock
}

pub struct DistributedLockInfo<'a> {
    pool: &'a PgPool
}

impl <'a> DistributedLock<'a> {

    pub fn new(pool: &PgPool, key: i64) -> DistributedLock {

        DistributedLock {
            pool,
            key,
            lock_type: LockType::SessionLock,
            is_shared: false
        }
    }

    pub fn new_with_attributes(pool: &PgPool, key: i64, lock_type: LockType, is_shared: bool) -> DistributedLock {

        DistributedLock {
            pool,
            key,
            lock_type,
            is_shared
        }
    }

    pub fn key(&self) -> i64 {
        self.key
    }

    pub fn lock_type(&self) -> &LockType {
        &self.lock_type
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

    pub async fn lock(&self) -> Result<(), sqlx::Error> {
        
        sqlx::query(&build_query(self.is_shared(), true))
            .bind(self.key())
            .execute(self.pool).await?;

        Ok(())
    }

    pub async fn try_lock(&self) -> Result<bool, sqlx::Error> {

        let locked: (bool, ) = sqlx::query_as(&build_query(self.is_shared(), false))
            .bind(self.key())
            .fetch_one(self.pool).await?;
    
        Ok(locked.0)
    }

    pub async fn unlock(&self) -> Result<(), sqlx::Error> {

        sqlx::query("SELECT pg_catalog.pg_advisory_unlock($1)")
            .bind(self.key())
            .execute(self.pool).await?;
    
        Ok(())
    }
}

impl <'a> DistributedLockInfo<'a> {

    pub async fn unlock_all(&self) {

        let _ = sqlx::query("SELECT pg_catalog.pg_advisory_unlock_all()")
            .execute(self.pool).await;
    }

    pub async fn is_locked(&self, key: i64) -> Result<bool, sqlx::Error> {

        let locked: (bool, ) = sqlx::query_as("SELECT EXISTS ( 
                SELECT objid FROM pg_catalog.pg_locks WHERE locktype = 'advisory' AND CAST(objid AS bigint) = $1 )")
            .bind(key)
            .fetch_one(self.pool).await?;

            Ok(locked.0)
    }

    pub async fn get_all_locks(&self) -> Result<Vec<i64>, sqlx::Error> {

        let locks: Vec<(i64,)> = sqlx::query_as("SELECT CAST(objid AS bigint) FROM pg_catalog.pg_locks WHERE locktype = 'advisory'")
            .fetch_all(self.pool).await?;

        // Convert Vec<(i64,)> to Vec<i64>
        let vec_of_i64: Vec<i64> = locks.into_iter().map(|(x,)| x).collect();

        Ok(vec_of_i64)
    }
}

fn build_query(is_shared: bool, is_wait: bool) -> String {

    let lock_type = if is_wait { "advisory_lock" } else { "try_advisory_lock" };
    let shared_part = if is_shared { "_shared" } else { "" };
    
    format!("SELECT pg_catalog.pg_{}{}($1)", lock_type, shared_part)
}