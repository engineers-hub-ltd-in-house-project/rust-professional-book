#include <iostream>
#include <vector>
#include <numeric>
#include <algorithm>
#include <chrono>

int main() {
    std::cout << "=== C++ Benchmark ===" << std::endl;
    
    std::vector<int> sizes = {1'000, 10'000, 100'000};
    
    for (int size : sizes) {
        auto start = std::chrono::high_resolution_clock::now();
        
        // データ生成
        std::vector<int> data(size);
        std::iota(data.begin(), data.end(), 0);
        
        // 処理: 偶数のみフィルタ → 2倍 → 合計
        long result = std::accumulate(data.begin(), data.end(), 0L,
            [](long sum, int x) {
                return sum + (x % 2 == 0 ? x * 2 : 0);
            });
        
        auto end = std::chrono::high_resolution_clock::now();
        auto elapsed = std::chrono::duration_cast<std::chrono::microseconds>(end - start);
        double elapsedMs = elapsed.count() / 1000.0;
        
        std::cout << "Size: " << size << ", Result: " << result 
                  << ", Time: " << elapsedMs << " ms" << std::endl;
    }
    
    return 0;
}