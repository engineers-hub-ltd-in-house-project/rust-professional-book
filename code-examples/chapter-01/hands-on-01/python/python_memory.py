import sys
import time
import tracemalloc

class Record:
    def __init__(self, id, name, value):
        self.id = id
        self.name = name
        self.value = value

def reference_counting_demo():
    """参照カウントの動作確認"""
    print("=== Reference Counting Demo ===")
    
    # オブジェクト作成
    record = Record(1, "test", 1.5)
    print(f"Reference count after creation: {sys.getrefcount(record) - 1}")  # -1は引数分
    
    # 参照追加
    another_ref = record
    print(f"Reference count after assignment: {sys.getrefcount(record) - 1}")
    
    # 参照削除
    del another_ref
    print(f"Reference count after deletion: {sys.getrefcount(record) - 1}")

def memory_pressure_test():
    """メモリ使用量測定"""
    print("\n=== Memory Pressure Test ===")
    tracemalloc.start()
    
    start_time = time.time()
    
    for iteration in range(1000):
        # 大量オブジェクト生成
        records = []
        for i in range(10000):
            records.append(Record(i, f"Record_{i}", i * 1.5))
        
        # 処理
        total = sum(record.value for record in records)
        
        if iteration % 100 == 0:
            current, peak = tracemalloc.get_traced_memory()
            print(f"Iteration {iteration}: {total:.2f}")
            print(f"Current memory: {current / 1024 / 1024:.2f} MB")
            print(f"Peak memory: {peak / 1024 / 1024:.2f} MB")
    
    end_time = time.time()
    print(f"Total time: {(end_time - start_time) * 1000:.2f} ms")
    
    tracemalloc.stop()

if __name__ == "__main__":
    reference_counting_demo()
    memory_pressure_test()