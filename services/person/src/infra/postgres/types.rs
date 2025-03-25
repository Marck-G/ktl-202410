use diesel::{r2d2::{ConnectionManager, PooledConnection}, PgConnection};

pub type PgConnectionPool = PooledConnection<ConnectionManager<PgConnection>>;

