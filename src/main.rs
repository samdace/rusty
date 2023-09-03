
use surrealdb::{Surreal, engine::local::RocksDb, opt::auth::Root };
use serde::{Serialize , Deserialize};
use surrealdb::opt::{Config};
use surrealdb::dbs::Capabilities;

#[derive(Serialize , Deserialize , Debug)]
struct Groupe {
    name:  String
}


#[derive(Serialize , Deserialize , Debug)]
struct User {
    username : String,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let config = Config::new()
    .capabilities(Capabilities::all())
    .user(Root{username : "root" , password : "root"});
    let db = Surreal::new::<RocksDb>(("data.db" , config)).await?;
    db.use_ns("test").use_db("test").await?;
    let _usercreated  : Vec<User> = db.create("user").content(User{username : "houssem".to_string()}).await?;
    let user : Vec<User> = db.select("user").await?;
    println!("user {:?}" , user);
    Ok(())

}
