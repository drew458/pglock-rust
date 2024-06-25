struct DistributedLock {
    key: i128,
    lock_type: LockType,
    is_shared: bool
}

enum LockType {
    SessionLock,
    TransactionLock
}

impl DistributedLock {

    pub fn new(key: i128, lock_type: LockType, is_shared: bool) {

        DistributedLock {
            key,
            lock_type,
            is_shared
        }
    }

    pub fn key(&self) -> &i128 {
        &self.key
    }

    pub fn lock_type(&self) -> &LockType {
        &self.loc
    }

    pub fn is_shared(&self) -> &bool {
        &self.key
    }
}