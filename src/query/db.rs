use super::table::{Table, TableCreate, TableDrop, TableList};
use super::term_type as ty;

pub fn db(name: &str) -> Db {
    Db { name: name.into_string() }
}

pub fn db_create(name: &str) -> DbCreate {
    DbCreate { name: name.into_string() }
}

pub fn db_drop(name: &str) -> DbDrop {
    DbDrop { name: name.into_string() }
}

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
