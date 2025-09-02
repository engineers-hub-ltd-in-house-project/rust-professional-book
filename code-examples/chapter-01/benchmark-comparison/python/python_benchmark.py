import time

def benchmark_python():
    print("=== Python Benchmark ===")
    
    sizes = [1_000, 10_000, 100_000]
    
    for size in sizes:
        start_time = time.time()
        
        # データ生成
        data = list(range(size))
        
        # 処理: 偶数のみフィルタ → 2倍 → 合計
        result = sum(x * 2 for x in data if x % 2 == 0)
        
        end_time = time.time()
        elapsed_ms = (end_time - start_time) * 1000
        
        print(f"Size: {size}, Result: {result}, Time: {elapsed_ms:.2f} ms")

if __name__ == "__main__":
    benchmark_python()