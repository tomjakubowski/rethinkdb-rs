import rethinkdb as r

r.connect('localhost').repl()
print("ReQL available on r")
