use serialize::json;

use super::Writes;
use super::db::Db;
use super::term_type as ty;
use super::cursor::Cursor;

query! {
    enum TableCreate -> () {
        TableCreate1 { name: String },
        TableCreate2 { db: Db, name: String }
    } ty::TABLE_CREATE
}

/// Create a new table in the connection's default database.
pub fn table_create(name: &str) -> TableCreate {
    TableCreate::TableCreate1 { name: name.into_string() }
}

query! {
    enum TableDrop -> () {
        TableDrop1 { name: String},
        TableDrop2 { db: Db, name: String }
    } ty::TABLE_DROP
}

/// Delete a table by name in the connection's default database.
pub fn table_drop(name: &str) -> TableDrop {
    TableDrop::TableDrop1 { name: name.into_string() }
}

query! {
    enum TableList -> Vec<String> {
        TableList1 { db: Db },
        TableList0
    } ty::TABLE_LIST
}

/// List all tables in the default database.
pub fn table_list() -> TableList {
    TableList::TableList0
}

/*
query! {
    enum Table -> Cursor {
        Table1 { name: String },
        Table2 { db: Db, name: String }
    } ty::TABLE
}
*/

// Do this by hand because of the lifetime parameter on Cursor
pub enum Table {
    Table1 { name: String },
    Table2 { db: Db, name: String }
}

impl ::query::Term for Table {
    fn args(&self) -> Vec<::serialize::json::Json> {
        use serialize::json::ToJson;
        match *self {
            Table::Table1 { ref name } => vec![name.to_json()],
            Table::Table2 { ref db, ref name } => vec![db.to_json(), name.to_json()]
        }
    }
}

to_json_impl! { Table ty::TABLE }

impl<'a> ::query::Query<'a, Cursor<'a>> for Table {
}

/// Select all documents in a table.  If executed as a query, return a `Cursor`
/// object which can iterate over all documents in the table.  Can also be
/// chained with the methods on `Table` to further process data in the table's
/// documents.
pub fn table(name: &str) -> Table {
    Table::Table1 { name: name.into_string() }
}

impl Table {
    pub fn get(self, key: &str) -> Get {
        Get { table: self, key: key.into_string() }
    }

    pub fn insert(self, document: json::Json) -> Insert {
        Insert { table: self, document: document }
    }

    pub fn index_create(self, name: &str) -> IndexCreate {
        IndexCreate { table: self, name: name.into_string() }
    }

    pub fn index_drop(self, name: &str) -> IndexDrop {
        IndexDrop { table: self, name: name.into_string() }
    }

    pub fn index_list(self) -> IndexList {
        IndexList { table: self }
    }
}

query! {
    // FIXME: Get should really be generic in its reql return type to support decoding
    // responses directly into structs
    Get -> json::Json {
        table: Table,
        key: String
    } ty::GET
}

impl Get {
    pub fn delete(self) -> Delete {
        Delete::DeleteGet { get: self }
    }
}

query! {
    Insert -> Writes {
        table: Table,
        document: json::Json
    } ty::INSERT
}

query! {
    // FIXME: this could also be generic over a type bounded by some trait
    // SingleSelection, on which the delete() method would be defined. But this means
    // users need to import that trait. Come back to this.
    enum Delete -> Writes {
        DeleteGet { get: Get }
    } ty::DELETE
}

query! {
    IndexCreate -> () {
        table: Table,
        name: String
    } ty::INDEX_CREATE
}

query! {
    IndexDrop -> () {
        table: Table,
        name: String
    } ty::INDEX_DROP
}

query! {
    IndexList -> Vec<String> {
        table: Table
    } ty::INDEX_LIST
}

