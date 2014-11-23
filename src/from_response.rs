use errors::RdbResult;
use net::{Connection, Response, ResponseKind};
use query;
use serialize::json;

pub trait FromResponse<'a> {
    fn from_response(Response, &'a mut Connection) -> RdbResult<Self>;
}

impl<'a> FromResponse<'a> for Vec<String> {
    fn from_response(res: Response, _: &'a mut Connection) -> RdbResult<Vec<String>> {
        use errors::Error::DriverError;

        match res.kind {
            ResponseKind::Atom => {
                // vvv FIXME bad unwraps
                let list = res.values.as_list().unwrap();
                let list = list[0].as_list().unwrap();

                Ok(list.iter().map(|s| {
                    s.as_string().unwrap().to_string()
                }).collect())
            },
            ResponseKind::Sequence => {
                Err(DriverError(format!("FIXME ResponseSequence {}", res)))
            },
            _ => {
                Err(DriverError(format!("Couldn't convert {} to Vec<String>", res)))
            }
        }
    }
}

impl<'a> FromResponse<'a> for json::Json {
    fn from_response(res: Response, _: &mut Connection) -> RdbResult<json::Json> {
        Ok(res.values)
    }
}

impl<'a> FromResponse<'a> for query::Writes {
    // vvvv this is all very very bad
    fn from_response(res: Response, _: &mut Connection) -> RdbResult<query::Writes> {
        use serialize::Decodable;
        let list = res.values.as_list().unwrap();
        let mut decoder = json::Decoder::new(list[0].clone()); // FIXME
        let insertion: query::Writes = Decodable::decode(&mut decoder).unwrap(); // FIXME
        Ok(insertion)
    }
}

impl<'a> FromResponse<'a> for () {
    fn from_response(_: Response, _: &mut Connection) -> RdbResult<()> { Ok(()) }
}
