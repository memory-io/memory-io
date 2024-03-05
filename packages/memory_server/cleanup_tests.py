import pymongo

myclient = pymongo.MongoClient("mongodb://localhost:27017/")
## drop all db that start with 'test'
for db in myclient.list_database_names():
    if db.startswith('test'):
        myclient.drop_database(db)

## drop all collection that start with 'test