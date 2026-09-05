
import os

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
        with open(f"{self.filename}.dbstuff", 'r') as fp:
            line_count = sum(1 for line in fp)
        return line_count  # next index = how many rows already exist 
    
    # adds a row to the end of the document   
    def addRow(self, data):
        with open(f'{self.filename}.dbstuff', 'a') as f:
            index = self._indexer()
            f.write(f"{index},{data}\n")
  
    # returns a row based on the index
    def readRow(self, index: int):
       with open(f'{self.filename}.dbstuff', 'r') as f:
            lines = f.readlines()
            for line in lines:
                if int(line[0]) == index:
                   return line[2:]

            return "index not found"


   
    def editRow(self,data, index = None , primeryKey = None ):
        pass

    #dumps DB
    def dumpDB(self):
        with open(f'{self.filename}.dbstuff', 'r') as f:
            dump = f.read()
            return dump



class AridKV():
    def __init__(self,filename = "database"):
        self.filename = filename

    def addRedcord(self,filename, Data: dict):
        print(filename,Data)

    def readRecord(self,filename,key):
        print(filename,key)



if __name__ == "__main__":

    database = AridDB()
    
    print(database.readRow(5))
    print(database.dumpDB)
    
    
