use actix_web::web::Json;
use actix_web::Responder;
use actix_web::web::Path;

use rand::Rng;

use rusqlite::{params, Connection, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    id: u32,
    name: String,
    price: u32,
    category: String,
}

pub async fn item_all() -> impl Responder {
    let items: Vec<Item> = vec![];
    Json(items)
}

pub fn prepare_database() -> Result<()> {
    let connection = Connection::open_in_memory()?;
    connection.execute(
        "CREATE TABLE T_ITEM (
                  ID              INTEGER PRIMARY KEY,
                  NAME            TEXT NOT NULL,
                  PRICE           INTEGER NOT NULL,
                  CATEGORY        TEXT NOT NULL
                  )",
        params![],
    )?;

    let mut rng = rand::thread_rng();
    for i in 0..10 {
        let item = Item {
            id: i,
            name: "FTX ".to_owned() + rng.gen_range(3800..4000).to_string().as_str(),
            price: rng.gen_range(1000..2500),
            category: "GPU".to_string(),
        };
        connection.execute(
            "INSERT INTO T_ITEM (NAME, PRICE, CATEGORY) VALUES (?1, ?2, ?3)",
            params![item.name, item.price, item.category],
        )?;
    }

    let mut statement = connection.prepare("SELECT ID, NAME, PRICE, CATEGORY FROM T_ITEM")?;
    let item_iter = statement.query_map(params![], |row| {
        Ok(Item {
            id: row.get(0)?,
            name: row.get(1)?,
            price: row.get(2)?,
            category: row.get(3)?,
        })
    })?;

    for item in item_iter {
        println!("Found item {:?}", item.unwrap());
    }
    Ok(())
}

#[derive(Deserialize)]
pub struct Info {
    id: u32,
}

pub async fn item_by_id(info: Path<Info>) -> impl Responder {
    Json(Item {
        id: info.id,
        name: String::from("New 980"),
        price: 55,
        category: String::from("SSD"),
    })
}