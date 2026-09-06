
from ast import literal_eval
import os
from collections import deque

class AridDB():
    
    def __init__(self,filename = "database"):
        self.filename = filename
        if self._dbExists():
            print("Database already exists")
        else:
            print("Creating new database")
            newDB = open(f"{self.filename}.dbstuff", "w")
            newDB.close()

        
    # checks if the db already exixts
    def _dbExists(self):
        if os.path.exists(f"{self.filename}.dbstuff"):
            return True
        else:
            return False

        
    # index number generator
    def _indexer(self):
        with open(f'{self.filename}.dbstuff', 'r') as f:
            # maxlen=1 ensures only the last line is kept in memory
            last_lines = deque(f, maxlen=1)
        if not last_lines:
            return 0
        last_line = last_lines.pop().strip()
        #blank space
        if not last_line:
            return 0
        return int(last_line[0]) + 1   
    
    # adds a row to the end of the document   
    def addRow(self, data: list):
        with open(f'{self.filename}.dbstuff', 'a') as f:
            index = self._indexer()
            f.write(f"{index},{data}\n")
            return f'Row added with index {index}'
  
    # returns a row based on the index
    def readRow(self, index: int):
       with open(f'{self.filename}.dbstuff', 'r') as f:
            lines = f.readlines()
            for line in lines:
                if int(line[0]) == index:
                   return literal_eval(line[2:])

            return "index not found"

    def deleteRow(self, index: int):
        with open(f'{self.filename}.dbstuff', 'r') as f:
            lines = f.readlines()

        new_lines = []
        found = False
        for line in lines:
            line_index, _ = line.split(',', 1)
            if int(line_index) == index:
                found = True
                continue  # skip this line -> it's removed
            new_lines.append(line)

        if not found:
            return "index not found"

        with open(f'{self.filename}.dbstuff', 'w') as f:
            f.writelines(new_lines)

        return "Row deleted"


   
    def editRow(self, data: list, index: int):
        with open(f'{self.filename}.dbstuff', 'r') as f:
            lines = f.readlines()

        new_lines = []
        found = False
        for line in lines:
            line_index, _ = line.split(',', 1)
            if int(line_index) == index:
                found = True
                new_lines.append(f"{index},{data}\n")
                continue
            new_lines.append(line)

        if not found:
            return "index not found"

        with open(f'{self.filename}.dbstuff', 'w') as f:
            f.writelines(new_lines)

        return "Row edited"

    #dumps DB
    def dumpDB(self):
        with open(f'{self.filename}.dbstuff', 'r') as f:
            return f.read()





if __name__ == "__main__":

    database = AridDB("mydatabase")
    print(database.readRow(0)[0])

    
