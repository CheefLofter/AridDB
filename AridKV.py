
from collections import deque
import os
from ast import literal_eval


class AridKV():
    def __init__(self,filename = "kvstore"):
        self.filename = filename
        if self._kvExists():
            print("kvstore already exists")
        else:
            print("Creating new kvstore")
            newDB = open(f"{self.filename}.kvstuff", "w")
            newDB.close()

        
    # checks if the db already exixts
    def _kvExists(self):
        if os.path.exists(f"{self.filename}.kvstuff"):
            return True
        else:
            return False

        
    # index number generator
    def _indexer(self):
        with open(f'{self.filename}.kvstuff', 'r') as f:
            # maxlen=1 ensures only the last line is kept in memory
            last_lines = deque(f, maxlen=1)
        if not last_lines:
            return 0
        last_line = last_lines.pop().strip()
        #blank space
        if not last_line:
            return 0
        return int(last_line[0]) + 1 


     
    def addRecord(self, data: dict):
        with open(f'{self.filename}.kvstuff', 'a') as f:
            index = self._indexer()
            f.write(f"{index},{data}\n")
            return f'Row added with index {index}'

    def readRecord(self,index: int):
        pass

    def deleteRecord(self,index: int):
        pass





if __name__ == "__main__":
    kv = AridKV()
    print(kv.addRecord({"name": "John", "age": 30}))


