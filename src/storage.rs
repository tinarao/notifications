use redis::{Connection, RedisError};

pub struct Storage {
    pub client: redis::Client,
}

impl Storage {
    pub fn new() -> Self {
        let client = match redis::Client::open("redis://127.0.0.1:6379/") {
            Ok(c) => c,
            Err(e) => panic!("failed to connect to redis: {}", e),
        };

        return Storage { client };
    }

    pub fn get_conn(&self) -> Result<Connection, RedisError> {
        self.client.get_connection()
    }
}

// Example
// let storage = Storage::new();
//     let mut con = match storage.get_conn() {
//         Ok(c) => c,
//         Err(e) => panic!("failed to get a connection: {}", e),
//     };
//
//     let _: () = con.set("hava", "nagila").unwrap();
//
//     let val: Result<String, RedisError> = con.get("hava");
//     match val {
//         Ok(v) => println!("v: {}", v),
//         Err(e) => println!("err: {}", e),
//     }
