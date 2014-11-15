import rethinkdb as r
import json
import sys

query = sys.argv[1]
result = json.dumps(eval("r." + query).build())
print("assert_eq!(r::{}.to_json(), json!({}));".format(query, result))
