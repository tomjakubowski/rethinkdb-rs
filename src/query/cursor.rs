use errors::RdbResult;
use from_response::FromResponse;
use net::Response;

use serialize::json::Json;
use std::iter::Iterator;
use std::{slice, vec};

// FIXME: needs to handle ResponsePartial et al. this may mean that that Cursor
// will need to implement a moving Iterator directly, but not sure
// FIXME: could this be parameterized on the type of the document? maybe best to
// leave that up to users
pub struct Cursor {
    docs: Vec<Json>,
}

pub struct Items<'a> {
    items: slice::Items<'a, Json>
}

pub struct MoveItems {
    items: vec::MoveItems<Json>
}

impl FromResponse for Cursor {
    fn from_response(res: Response) -> RdbResult<Cursor> {
        use errors::Error::DriverError;

        debug!("Response: {}", res.values.to_pretty_str());
        let docs = match res.values.as_list() {
            Some(docs) => docs.clone(),
            None => return Err(DriverError("expected list".into_string()))
        };
        Ok(Cursor {
            docs: docs
        })
    }
}

impl Cursor {
    pub fn iter(&self) -> Items {
        Items {
            items: self.docs.iter()
        }
    }

    pub fn into_iter(self) -> MoveItems {
        MoveItems {
            items: self.docs.into_iter()
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

    fn fixture() -> Vec<Json> {
        vec![Person { name: "bob", age: 23 }.to_json(),
             Person { name: "sally", age: 25 }.to_json()]

    }

    #[test]
    fn test_iter() {
        let cursor = Cursor { docs: fixture() };
        let peeps = fixture();
        assert_eq!(peeps, cursor.iter().map(|x| x.clone()).collect())
    }

    #[test]
    fn test_into_iter() {
        let cursor = Cursor { docs: fixture() };
        let peeps = fixture();
        assert_eq!(peeps, cursor.into_iter().collect())
    }
}
