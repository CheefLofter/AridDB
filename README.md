# AridDB
**WIP**  A simple embeddable database and key-value store used to store data in local file system without the need of SQL. data cane be stored or retrieved using simple built-in functions.

## Usage



**Complete Functions:**
- `__init__(filename)` - Initialize database when(name = AridDB())
- `_dbExists()` - (checks if db exists already)
- `_indexer()` - Get next available index(not for outside use)
- `addRow(data)` - Add a new row with auto-generated index
- `readRow(index)` - read singe row (takes index value)
- `dumpDB()` - dumps db(doesnot work)

**Incomplete Functions:**
- `editRow(data, index, primeryKey)` - ⚠️ NOT IMPLEMENTED (only has pass statement)

### AridKV Class (Key-Value Store)

**Incomplete Functions:**
- `addRedcord(filename, Data)` - ⚠️ NOT IMPLEMENTED (only prints arguments, needs actual storage logic)
- `readRecord(filename, key)` - ⚠️ NOT IMPLEMENTED (only prints arguments, needs actual retrieval logic)

