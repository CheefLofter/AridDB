
import os

class AridDB():
    
    def __init__(self,filepath = "database"):
        self.filepath = filepath

    def _dbExists(self):
        if os.path.exists(self.filename):
            return True
        else:
            return False
        
        
    def addRow(self, data):
        print(data)
  

    def readRow(self, index = None,primeryKey = None):
        print(index, primeryKey)

    def editRow(self,data, index = None , primeryKey = None ):
        print(primeryKey,index,data)

    def dumpDB():
        print("db dumped")



class AridKV():
    def __init__(self,filepath = "database"):
        self.filepath = filepath

    def addRedcord(self,filepath, Data: dict):
        print(filepath,Data)

    def readRecord(self,filepath,key):
        print(filepath,key)



if __name__ == "__main__":

    database = AridDB()
    database.addRow(["asds","dsada"])