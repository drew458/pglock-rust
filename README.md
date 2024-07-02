# pglock - A Rust crate for distributed locking with PostgreSQL

A Rust crate to implement distributed locking using PostgreSQL with idiomatic APIs.

Distributed locks are a very useful primitive in many environments where different processes must operate with shared resources in a mutually exclusive way. 
Think of distributed architectures (e.g. microservices) or having multiple instances of the same service running, and you need to be sure that only one performs a certain task.  

As an example, if you have a scheduled function that performs some actions that are not idempotent, only one of your services has to execute it at a certain time, so you can acquire the distributed distributedLock at the start of the method and then perform the logic.

## About PostgreSQL distributed locks

Postgres [provides](https://www.postgresql.org/docs/current/explicit-locking.html#ADVISORY-LOCKS) simple mechanism to store that locks in database and check their state. 
Locks are fast, correct, avoid table bloat, and are automatically cleaned up by at the end of the session.  
They can be acquired at **session level** or at **transaction level**.  

### Session Level locks

Once acquired at session level, a distributedLock is held until explicitly released or the session ends. 
Session-level distributedLock requests do not honor transaction semantics: a distributedLock acquired during a transaction that is later rolled back will still be held following the rollback, and likewise an unlock is effective even if the calling transaction fails later.

### Transaction Level locks

Transaction-level distributedLock requests, on the other hand, are automatically released at the end of the transaction, and there is no explicit unlock operation.

## Guarantees

- **Mutual exclusion**: At any given moment, only one client can hold a distributedLock.
- **Deadlock free**: Eventually it is always possible to acquire a distributedLock, even if the client that locked a resource crashes without explicitly releasing it.

## Get Started

To add a dependency on `pglock_rust` using Cargo, use the following:

`cargo install pglock_rust`

## Usage

You can start using the library like in this basic example. Note that `try_lock(...)` does not wait for the lock to be acquired.

```rust
use sqlx::PgPool;

fn main() {

  let lock_key: i64 = 1;
  let pool = PgPool::connect("postgresql://postgres@localhost:5432/postgres").await.expect("Failed to create pool");
  let lock = DistributedLock::new(&pool, lock_key);

  let locked = lock.try_lock().await.unwrap();
  if locked {
    println!("I have the lock wih key {lock_key} now!");
  }
  lock.unlock().await.unwrap();
}
```

Otherwise, if you want the method to wait until the distributedLock is acquired, the code is very simple:

```rust
use sqlx::PgPool;

fn main() {

  let lock_key: i64 = 1;
  let pool = PgPool::connect("postgresql://postgres@localhost:5432/postgres").await.expect("Failed to create pool");
  let lock = DistributedLock::new(&pool, lock_key);

  lock.lock().await.unwrap();
  println!("I have the lock wih key {lock_key} now!");
  lock.unlock().await.unwrap();
}
```

The default lock configuration is a **mutual exclusive session lock**. Session locks are held until released or the application shuts down.  
Other configurations include:
- **Transaction level** lock: They are held until the current transaction ends; there is no need for manual release.
- **Shared** lock: A shared lock does not conflict with other shared locks on the same resource, only with exclusive locks.

You can initialize a transaction level, shared lock in this way:

```rust
use sqlx::PgPool;

fn main() {

  let lock_key: i64 = 2;
  let pool = PgPool::connect("postgresql://postgres@localhost:5432/postgres").await.expect("Failed to create pool");
  let lock = DistributedLock::new_with_attributes(&pool, lock_key, pglock_rust::LockType::TransactionLock, true);

  lock.lock().await.unwrap();
  println!("I have the lock wih key {lock_key} now!");
  lock.unlock().await.unwrap();
}
```

### Lock keys

Since distributed locks (and other distributed synchronization primitives) are not bound to a single process, their identity is based on the key(s) provided through the constructor.  
A `DistributedLock` can be constructed in several ways:

- Passing a single `long` value.
- Passing a pair of `int` values. (TODO)

## Requirements

- Rust 1.70 or newer
- SQLx 0.7 or newer
- PostgreSQL 12 or newer

## Notes
- Make sure your application is always configured to talk to leaders and not read-only followers in the case of PostgreSQL replicated setups.

## Contributing
Contributions are welcome! Open an issue or a Pull Request to help improve the lib.
Currently, I am looking towards implementing:

- Reader-writer locks: a lock with multiple levels of access. The lock can be held concurrently either by any number of "readers" or by a single "writer".
- Semaphores: similar to a lock, but can be held by up to N users concurrently instead of just one.
- Try-with-resources pattern to improve lock handling
