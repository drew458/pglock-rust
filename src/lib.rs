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

    pub fn new(key: i128, lock_type: LockType, is_shared: bool) -> DistributedLock {

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
        &self.lock_type
    }

    pub fn is_shared(&self) -> &bool {
        &self.is_shared
    }

    pub fn set_key(&mut self, key: i128) {
        self.key = key
    }

    pub fn set_lock_type(&mut self, lock_type: LockType) {
        self.lock_type = lock_type
    }

    pub fn set_is_shared(&mut self, is_shared: bool) {
        self.is_shared = is_shared
    }
}