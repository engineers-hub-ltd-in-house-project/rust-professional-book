import java.util.*;
import java.util.stream.IntStream;

public class JavaBenchmark {
    public static void main(String[] args) {
        System.out.println("=== Java Benchmark ===");
        
        int[] sizes = {1_000, 10_000, 100_000};
        
        for (int size : sizes) {
            long startTime = System.nanoTime();
            
            // データ生成
            List<Integer> data = IntStream.range(0, size)
                .boxed()
                .collect(ArrayList::new, ArrayList::add, ArrayList::addAll);
            
            // 処理: 偶数のみフィルタ → 2倍 → 合計
            int result = data.stream()
                .filter(x -> x % 2 == 0)
                .mapToInt(x -> x * 2)
                .sum();
            
            long endTime = System.nanoTime();
            double elapsedMs = (endTime - startTime) / 1_000_000.0;
            
            System.out.printf("Size: %d, Result: %d, Time: %.2f ms\n", 
                size, result, elapsedMs);
        }
    }
}