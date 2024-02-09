


use std::error::Error;
use sqlx::Connection;
use sqlx::Row;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let url = "postgres://postgres:sjualyoacp@localhost:5432/postgres";
   let pool = sqlx::postgres::PgPool::connect(url).await?;

   sqlx::migrate!("./migrations").run(&pool).await?;
  let res = sqlx::query("select 1+1 as sum")
             .fetch_one(&pool)
             .await?;
   let sum:i32 = res.get("sum");
   println!("1 + 1 = {}", sum);
   Ok(())
}


sulochanajayaprakash@Sulochanas-MacBook-Pro yahoo1 % PGPASSWORD="sjualyoacp" psql -U postgres -h 0.0.0.0 -p 5432 postgres
psql (16.0, server 16.1)
psql (16.0, server 16.1)
Type "help" for help.

postgres=# \dt
              List of relations
 Schema |       Name       | Type  |  Owner   
--------+------------------+-------+----------
 public | _sqlx_migrations | table | postgres
 public | author           | table | postgres
 public | book             | table | postgres
 public | book1            | table | postgres
 public | livedata1        | table | postgres
(5 rows)






