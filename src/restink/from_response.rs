use super::net::{mod, Connection, RdbResult, Response};
use super::query;
use serialize::json::{mod, ToJson};

trait FromResponse {
    fn from_response(res: Response) -> RdbResult<Self>;
}

impl FromResponse for Vec<String> {
    fn from_response(res: Response) -> RdbResult<Vec<String>> {
        use net::{DriverError, ResponseAtom, ResponseSequence};

        match res.kind {
            ResponseAtom => {
                // vvv bad
                let list = res.values.as_list().unwrap();
                let list = list[0].as_list().unwrap();

                Ok(list.iter().map(|s| {
                    s.as_string().unwrap().to_string()
                }).collect())
            },
            ResponseSequence => {
                Err(DriverError(format!("FIXME ResponseSequence {}", res)))
            },
            _ => {
                Err(DriverError(format!("Couldn't convert {} to Vec<String>", res)))
            }
        }
    }
}

impl FromResponse for query::Writes {
    // vvvv this is all very very bad
    fn from_response(res: Response) -> RdbResult<query::Writes> {
        use serialize::Decodable;
        let list = res.values.as_list().unwrap();
        let mut decoder = json::Decoder::new(list[0].clone()); // FIXME
        let insertion: query::Writes = Decodable::decode(&mut decoder).unwrap(); // FIXME
        Ok(insertion)
    }
}

impl FromResponse for Response {
    fn from_response(res: Response) -> RdbResult<Response> { Ok(res) }
}

impl FromResponse for json::Json {
    fn from_response(res: Response) -> RdbResult<json::Json> { Ok(res.values) }
}

impl FromResponse for query::Document {
    fn from_response(res: Response) -> RdbResult<query::Document> {
        Ok(query::Document(res.values))
    }
}

impl FromResponse for () {
    fn from_response(_: Response) -> RdbResult<()> { Ok(()) }
}

impl<Out: FromResponse> query::Func<Out> {
    pub fn run(self, conn: &mut Connection) -> RdbResult<Out> {
        net::run(conn, self.to_json())
            .and_then(FromResponse::from_response)
    }
}
