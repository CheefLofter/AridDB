# Arid DB
The AridDB class comes with multiple function to retrive and add data to the file. The files will automatically have the default extention of .dbstf and .kvstf(for kv store). the initial start will check if the file exists and will use it instead of creating a new file. *Note: the file names must be same for the db to recognize the pre-existing file*. 

           
- Initialization
```
import AridDB
# creates a datbase.dbstf file in the same dir
db = AridDB()
# if you dont want the default name you can specify it as well
db = AridDB("Datastore")
```

- Adding new row
```
db.addRow(data; list)
```
- Adding multiple new rows 
```
db.addRows(data; list)
#takes a 2d list of rows 
```

- Reading row data 
```
db.readRow(index: int)
# reads a row for the give index
```

- Deleting row
```
db.deleteRow(index: int)
# deletes the row at given index 

```
- Editing rows
```
db.editRow(data: list, index: int)
# replaces the data at given index with data provided
```

- dump entire db
```
db.dumDB()
# dumps the contents of the database with the indexes 
```

# Arid KV