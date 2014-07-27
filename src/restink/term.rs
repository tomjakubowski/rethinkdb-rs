use serialize::json;
use serialize::json::{Json,ToJson};

pub type Datum = json::Json;

pub struct Func<Out> {
    func_type: FuncType,
    prev: Option<Json>,
    args: Vec<Datum>,
    opt_args: Option<json::Object>
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

impl<T> ToJson for Func<T> {
    fn to_json(&self) -> json::Json {
        use std::collections::TreeMap;

        let Func { func_type, ref prev, ref args, ref opt_args } = *self;
        let mut term_args = match *prev {
            Some(ref f) => { vec![f.to_json()] },
            None => { vec![] }
        };
        term_args.push_all(args.as_slice());

        let term_opt_args = match *opt_args {
            None => json::Object(TreeMap::new()),
            Some(ref ob) => json::Object(ob.clone())
        };

        json::List(vec![func_type.to_json(), term_args.to_json(), term_opt_args.to_json()])
    }
}

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
        json::Number(*self as u32 as f64)
    }
}
