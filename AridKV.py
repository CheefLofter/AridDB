
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
            newDB = open(f"{self.filename}.kvstf", "w")
            newDB.close()

        
    # checks if the db already exixts
    def _kvExists(self):
        if os.path.exists(f"{self.filename}.kvstf"):
            return True
        else:
            return False

        
    # index number generator
    def _indexer(self):
        with open(f'{self.filename}.kvstf', 'r') as f:
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
    
        with open(f'{self.filename}.kvstf', 'a') as f:
            index = self._indexer()
            f.write(f"{index},{data}\n")
            return f'Record added with index {index}'

    def readRecord(self,index: int) -> dict:
       with open(f'{self.filename}.kvstf', 'r') as f:
            lines = f.readlines()
            for line in lines:
                if int(line[0]) == index:
                   return literal_eval(line[2:])

            return "index not found"

    def deleteRecord(self,index: int):
        with open(f'{self.filename}.kvstf', 'r') as f:
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

        with open(f'{self.filename}.kvstf', 'w') as f:
            f.writelines(new_lines)

        return "Record deleted"



    def editRecord(self,index:int,data:dict):
        with open(f'{self.filename}.kvstf', 'r') as f:
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

        with open(f'{self.filename}.kvstf', 'w') as f:
            f.writelines(new_lines)

        return "Record edited"


    def dumpRecords(self):
        with open(f'{self.filename}.kvstf', 'r') as f:
            return f.read()
        



if __name__ == "__main__":
    kv = AridKV()
    print(kv.dumpRecords())
  


