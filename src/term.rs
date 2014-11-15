use serialize::json;
use serialize::json::{Json,ToJson};

use RdbResult;
use from_response::FromResponse;
use net::{mod, Connection};

pub type Datum = json::Json;

pub struct Func<Out> {
    func_type: FuncType,
    prev: Option<Json>,
    args: Vec<Datum>,
    opt_args: Option<json::JsonObject>
}

impl<Out, In> Func<In> {
    pub fn chain(self, func_type: FuncType, args: Vec<Datum>) -> Func<Out> {
        Func {
            func_type: func_type,
            prev: Some(self.to_json()),
            args: args,
            opt_args: None
        }
    }
}

impl<Out> Func<Out> {
    pub fn start(func_type: FuncType, args: Vec<Datum>) -> Func<Out> {
        Func {
            func_type: func_type,
            prev: None,
            args: args,
            opt_args: None
        }
    }
}

impl<Out: FromResponse> Func<Out> {
    pub fn run(self, conn: &mut Connection) -> RdbResult<Out> {
        net::run(conn, self.to_json()).and_then(FromResponse::from_response)
    }
}

impl<T> ToJson for Func<T> {
    fn to_json(&self) -> json::Json {
        let Func { func_type, ref prev, ref args, ref opt_args } = *self;
        let mut term_args = match *prev {
            Some(ref f) => { vec![f.to_json()] },
            None => { vec![] }
        };
        term_args.push_all(args.as_slice());

        match *opt_args {
            None => (func_type.to_json(), term_args.to_json()).to_json(),
            Some(ref ob) => {
                (func_type.to_json(), term_args.to_json(), ob.to_json()).to_json()
            }
        }
    }
}

#[repr(i64)]
pub enum FuncType {
    Db = 14,
    Table = 15,
    Get = 16,
    Insert = 56,
    TableCreate = 60,
    TableDrop = 61,
    TableList = 62,
    IndexCreate = 75,
    IndexDrop = 76,
    IndexList = 77
}

impl ToJson for FuncType {
    fn to_json(&self) -> json::Json {
        json::I64(*self as i64)
    }
}
