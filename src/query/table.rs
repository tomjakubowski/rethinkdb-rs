use serialize::json;

use super::{Db, Writes};
use super::term_type as ty;

term! {
    enum TableCreate -> () {
        TableCreate1 { name: String },
        TableCreate2 { db: Db, name: String }
    } ty::TABLE_CREATE
}

pub fn table_create(name: &str) -> TableCreate {
    TableCreate1 { name: name.into_string() }
}

term! {
    enum TableDrop -> () {
        TableDrop1 { name: String},
        TableDrop2 { db: Db, name: String }
    } ty::TABLE_DROP
}

pub fn table_drop(name: &str) -> TableDrop {
    TableDrop1 { name: name.into_string() }
}

term! {
    enum TableList -> Vec<String> {
        TableList1 { db: Db },
        TableList0
    } ty::TABLE_LIST
}

pub fn table_list() -> TableList {
    TableList0
}

term! {
    // FIXME: should return an iterator over the documents of the table
    enum Table -> () {
        Table1 { name: String },
        Table2 { db: Db, name: String }
    } ty::TABLE
}

pub fn table(name: &str) -> Table {
    Table1 { name: name.into_string() }
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

term! {
    Get -> json::Json {
        table: Table,
        key: String
    } ty::GET
}

term! {
    Insert -> Writes {
        table: Table,
        document: json::Json
    } ty::INSERT
}

term! {
    IndexCreate -> () {
        table: Table,
        name: String
    } ty::INDEX_CREATE
}

term! {
    IndexDrop -> () {
        table: Table,
        name: String
    } ty::INDEX_DROP
}

term! {
    IndexList -> Vec<String> {
        table: Table
    } ty::INDEX_LIST
}

