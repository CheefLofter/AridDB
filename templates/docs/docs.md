# AridDB Documentation

The AridDB library provides a lightweight file-based database system for storing rows as Python lists. Each database is saved in a plain-text file using the `.dbstf` extension, and the class automatically reuses an existing database with the same filename instead of creating a new one.

Note: the filename must match the existing file name to load the same database.

## Class: AridDB

### Initialization

```python
import AridDB

# Creates a database.dbstf file in the current directory
DB = AridDB.AridDB()

# Use a custom filename
DB = AridDB.AridDB("Datastore")
```

When initialized:
- If `database.dbstf` already exists, it opens that file.
- If it does not exist, it creates a new empty file.

### Methods

#### `addRow(data: list)`
Adds a single row to the end of the database.

```python
DB.addRow(["Alice", 21, "New York"])
```

This adds the row and returns a message confirming the new index.

#### `addRows(data: list)`
Adds multiple rows at once. Each item in the list must itself be a list.

```python
rows = [
    ["Bob", 25, "London"],
    ["Charlie", 30, "Tokyo"]
]

DB.addRows(rows)
```

This adds all rows in order and returns a confirmation message.

#### `readRow(index: int)`
Reads and returns the row stored at the given index.

```python
row = DB.readRow(0)
```

This returns the matching row for that index, or a not found message if the index does not exist.

#### `deleteRow(index: int)`
Deletes a row at the specified index.

```python
DB.deleteRow(1)
```

This removes the selected row and returns a confirmation message if it was found.

#### `editRow(data: list, index: int)`
Replaces the row at the given index with the new data.

```python
DB.editRow(["Alice", 22, "Paris"], 0)
```

This updates the row at the selected index and returns a confirmation message.

#### `dumpDB()`
Returns the full contents of the database file, including row indexes.

```python
DB.dumpDB()
```

This returns the database content as plain text, showing each index and its stored row.

### File Format
The database stores each row as:
```text
<index>,<python_list_literal>
```

Example:
```text
0,['Alice', 21, 'New York']
```

---

# AridKV Documentation

The AridKV class provides a lightweight key-value store stored in a plain-text file using the `.kvstf` extension. It stores records as Python dictionaries and supports reading by index and by key.

## Class: AridKV

### Initialization

```python
import AridKV

# Creates a kvstore.kvstf file in the current directory
KV = AridKV.AridKV()

# Custom filename
KV = AridKV.AridKV("user_data")
```

When initialized:
- If the file already exists, it uses it.
- If it does not exist, it creates a new empty file.

### Methods

#### `addRecord(data: dict)`
Adds a dictionary record to the store.

```python
KV.addRecord({"name": "Alice", "age": 21})
```

This adds the record and returns a confirmation message with the assigned index.

#### `readRecord(index: int) -> dict`
Reads a record by its numeric index.

```python
record = KV.readRecord(0)
```

This returns the matching record for that index, or a not found message if it does not exist.

#### `deleteRecord(index: int)`
Deletes a record at the specified index.

```python
KV.deleteRecord(0)
```

This removes the chosen record and returns a confirmation message if it was found.

#### `editRecord(index: int, data: dict)`
Replaces the record at the given index with a new dictionary.

```python
KV.editRecord(0, {"name": "Alice", "age": 22})
```

This updates the selected record and returns a confirmation message.

#### `readByKey(key: str)`
Searches through all records and returns the value for the first matching dictionary key.

```python
KV.readByKey("name")
```

This returns the value for the matching key, or a not found message if the key does not exist.

#### `dumpRecords()`
Returns the full contents of the key-value store file.

```python
KV.dumpRecords()
```

This returns the saved records as plain text, including their indexes.

### File Format
Each record is stored as:
```text
<index>,<python_dict_literal>
```

Example:
```text
0,{'name': 'Alice', 'age': 21}
```

---

## Notes
- Indexing starts at `0`.
- The library stores data as plain text, so it is best suited for small personal databases and learning projects.
- Data is read back using Python's `ast.literal_eval`, which safely converts stored literals back into Python objects.
- The filename used when creating an instance must stay consistent if you want to reopen the same database later.
