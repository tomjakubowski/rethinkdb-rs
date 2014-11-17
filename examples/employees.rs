#![feature(if_let, phase)]

#[phase(plugin)] extern crate json_macros;
extern crate rethinkdb;
extern crate serialize;

use serialize::json::{mod, ToJson};

use rethinkdb::{Connection, RdbResult};
use rethinkdb::query as r;
use rethinkdb::Query;

#[deriving(Decodable, Encodable, Show)]
struct Employee {
    id: Option<String>,
    name: String,
    catchphrase: String
}

impl Employee {
    fn new(name: &str, catchphrase: &str) -> Employee {
        Employee {
            id: None,
            name: name.into_string(),
            catchphrase: catchphrase.into_string()
        }
    }

    fn find_by_id(id: &str, conn: &mut Connection) -> RdbResult<Employee> {
        use serialize::Decodable;

        let table = r::table("employees");
        // see the FIXME on Get
        let docs = try!(table.get(id).run(conn));
        let mut decoder = json::Decoder::new(docs);

        let mut employees: Vec<Employee> = Decodable::decode(&mut decoder).unwrap();
        Ok(employees.remove(0).unwrap())
    }

    fn is_new(&self) -> bool {
        self.id.is_none()
    }

    fn delete(self, conn: &mut Connection) -> RdbResult<()> {
        let table = r::table("employees");
        let id = self.id.expect("Employee record has None id");
        table.get(id.as_slice()).delete().run(conn).and_then(|_| Ok(()))
    }

    fn save(&mut self, conn: &mut Connection) -> RdbResult<()> {
        let table = r::table("employees");
        if self.is_new() {
            let writes = try!(table.insert(self.to_json()).run(conn));
            self.id = writes.generated_keys.and_then(|mut x| x.remove(0));
        } else {
            // FIXME: update
        }
        Ok(())
    }
}

impl ToJson for Employee {
    fn to_json(&self) -> json::Json {
        if let Some(ref id) = self.id {
            json!({
                "id": (id),
                "name": (self.name),
                "catchphrase": (self.catchphrase)
            })
        } else {
            json!({
                "name": (self.name),
                "catchphrase": (self.catchphrase)
            })
        }
    }
}

fn it_stinks(conn: &mut Connection) -> RdbResult<()> {
    // NOTE: This is cheating. Once we can pick the key for get() calls, then we can
    // find_by_name "Jay Sherman".
    let jay_id = {
        let mut jay: Employee = Employee::new("Jay Sherman", "It stinks!");
        println!("Saving Jay Sherman...");
        try!(jay.save(conn));
        println!("Jay Sherman: {}", jay);
        jay.id
    }.expect("jay.id is None after save");

    println!("Loading fresh Jay Sherman from db...");
    let jay = try!(Employee::find_by_id(jay_id.as_slice(), conn));
    println!("Jay Sherman: {}", jay);

    // FIXME: update catchphrase to "Buy my book" + reload

    println!("Deleting Jay Sherman...");
    try!(jay.delete(conn));

    Ok(())
}

fn setup() -> RdbResult<Connection> {
    let mut conn = try!(rethinkdb::connect("127.0.0.1", 28015));
    conn.use_db("test");
    let tables = try!(r::table_list().run(&mut conn));

    if let None = tables.iter().find(|x| x.as_slice() == "employees") {
        println!("Creating employees table...");
        try!(r::table_create("employees").run(&mut conn));
        try!(r::table("employees").index_create("name").run(&mut conn));

        let indexes = r::table("employees").index_list().run(&mut conn);
        println!("Created table and indexes: {}", indexes);
    }

    Ok(conn)
}

pub fn main() {
    let conn = setup();
    if let Err(e) = conn {
        println!("Setup error: {}", e);
        return;
    };
    let mut conn = conn.unwrap();

    if let Err(e) = it_stinks(&mut conn) {
        println!("There was an error: {}", e);
    }
}
