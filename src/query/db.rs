use super::table::{Table, TableCreate, TableDrop, TableList};
use super::term_type as ty;

/// Select a database to act on.
pub fn db(name: &str) -> Db {
    Db { name: name.into_string() }
}

/// Create a new database.
pub fn db_create(name: &str) -> DbCreate {
    DbCreate { name: name.into_string() }
}

/// Delete an existing database.
pub fn db_drop(name: &str) -> DbDrop {
    DbDrop { name: name.into_string() }
}

/// List all database names in the system.
///
/// # Example
///
/// ```no_run
/// use rethinkdb::query as r;
/// use rethinkdb::Query;
/// # let mut conn = rethinkdb::connect("localhost", 1234).unwrap();
/// let dbs = r::db_list().run(&mut conn).unwrap();
/// for db in dbs.iter() {
///     println!("{}", db);
/// }
/// ```
pub fn db_list() -> DbList {
    DbList
}

term! {
    Db {
        name: String
    } ty::DB
}

impl Db {
    pub fn table(self, name: &str) -> Table {
        Table::Table2 { db: self, name: name.into_string() }
    }

    pub fn table_create(self, name: &str) -> TableCreate {
        TableCreate::TableCreate2 { name: name.into_string(), db: self }
    }

    pub fn table_drop(self, name: &str) -> TableDrop {
        TableDrop::TableDrop2 { name: name.into_string(), db: self }
    }

    pub fn table_list(self) -> TableList {
        TableList::TableList1 { db: self }
    }
}

query! {
    DbCreate -> () {
        name: String
    } ty::DB_CREATE
}

query! {
    DbDrop -> () {
        name: String
    } ty::DB_DROP
}

query! {
    DbList -> Vec<String> ; ty::DB_LIST
}
