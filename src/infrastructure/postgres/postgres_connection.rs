use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use anyhow::Result;
pub type Pgpoolsquad = Pool<ConnectionManager<PgConnection>>;
pub fn establish_connection(database_url: &str) -> Result<Pgpoolsquad> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager)?;
        
    Ok(pool)
}
