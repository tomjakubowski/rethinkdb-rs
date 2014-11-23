use errors::RdbResult;
use from_response::FromResponse;
use net::{Connection, Response};

use serialize::json::Json;
use std::iter::Iterator;
use std::{slice, vec};

// FIXME: needs to handle ResponsePartial et al. this may mean that that Cursor
// will need to implement a moving Iterator directly, but not sure
// FIXME: could this be parameterized on the type of the document? maybe best to
// leave that up to users
pub struct Cursor<'a> {
    /// The most recently-received chunk of documents from the conn
    chunk: Vec<Json>,
    conn: Option<&'a mut Connection>
}

pub struct Items<'a> {
    items: slice::Items<'a, Json>
}

pub struct MoveItems {
    items: vec::MoveItems<Json>
}

impl<'a> FromResponse<'a> for Cursor<'a> {
    fn from_response(res: Response, conn: &'a mut Connection) -> RdbResult<Cursor<'a>> {
        use errors::Error::DriverError;
        use net::ResponseKind;

        debug!("Response: {}", res);
        let chunk = match res.values.as_array() {
            Some(chunk) => chunk.clone(),
            None => return Err(DriverError("expected list".into_string()))
        };
        match res.kind {
            ResponseKind::Sequence => {
                Ok(Cursor {
                    chunk: chunk,
                    conn: None,
                })
            }
            ResponseKind::Partial => {
                Ok(Cursor {
                    chunk: chunk,
                    conn: Some(conn)
                })
            }
            ResponseKind::Atom => {
                Err(DriverError("unexpected SUCCESS_ATOM".into_string()))
            }
        }
    }
}

impl<'a> Cursor<'a> {
    pub fn iter(&self) -> Items {
        match self.conn {
            Some(_) => panic!("iterating over partial responses unimplemented"),
            None => Items { items: self.chunk.iter() }
        }
    }

    pub fn into_iter(self) -> MoveItems {
        match self.conn {
            Some(_) => panic!("iterating over partial responses unimplemented"),
            None => MoveItems { items: self.chunk.into_iter() }
        }
    }
}

impl<'a> Iterator<&'a Json> for Items<'a> {
    fn next(&mut self) -> Option<&'a Json> {
        self.items.next()
    }
}

impl Iterator<Json> for MoveItems {
    fn next(&mut self) -> Option<Json> {
        self.items.next()
    }
}

#[cfg(test)]
mod test {
    use super::Cursor;
    use serialize::json::{Json, ToJson};

    struct Person {
        name: &'static str,
        age: i32
    }

    impl ToJson for Person {
        fn to_json(&self) -> Json {
            json!({
                "name": (self.name),
                "age": (self.age)
            })
        }
    }

    struct Fixture<'a> {
        peeps: Vec<Json>,
        cursor: Cursor<'a>
    }

    fn fixture_data() -> Vec<Json> {
        vec![Person { name: "bob", age: 23 }.to_json(),
             Person { name: "sally", age: 25 }.to_json()]
    }

    fn fixture<'a>() -> Fixture<'a> {
        Fixture {
            peeps: fixture_data(),
            cursor: Cursor { chunk: fixture_data(), conn: None }
        }
    }

    #[test]
    fn test_iter() {
        let fix = fixture();
        assert_eq!(fix.peeps, fix.cursor.iter().map(|x| x.clone()).collect())
    }

    #[test]
    fn test_into_iter() {
        let fix = fixture();
        assert_eq!(fix.peeps, fix.cursor.into_iter().collect())
    }
}
