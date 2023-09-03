
use surrealdb::{Surreal, engine::local::File, opt::auth::Root };
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
    let path = format!("/tmp/{}.db", "data");
	let root = Root {
				username: "root",
				password: "root",
			};
	let config = Config::new()
				.user(root)
				.capabilities(Capabilities::all());
	let db = Surreal::new::<File>((path, config)).await.unwrap();
    db.use_ns("test").use_db("test").await?;
    let _usercreated  : Vec<User> = db.create("user").content(User{username : "houssem".to_string()}).await?;
    let user : Vec<User> = db.select("user").await?;
    println!("user {:?}" , user);
    Ok(())

}
