import java.util.*;

class Record {
    int id;
    String name;
    double value;
    
    Record(int id, String name, double value) {
        this.id = id;
        this.name = name;
        this.value = value;
    }
}

public class GcBenchmark {
    private static final int ITERATIONS = 1000;
    private static final int RECORDS_PER_ITERATION = 10000;
    
    public static void main(String[] args) {
        // GC圧力測定
        System.out.println("Starting GC benchmark...");
        
        long startTime = System.nanoTime();
        
        for (int iter = 0; iter < ITERATIONS; iter++) {
            List<Record> records = new ArrayList<>();
            
            // 大量オブジェクト生成
            for (int i = 0; i < RECORDS_PER_ITERATION; i++) {
                records.add(new Record(i, "Record_" + i, i * 1.5));
            }
            
            // 処理
            double total = records.stream()
                .mapToDouble(r -> r.value)
                .sum();
                
            if (iter % 100 == 0) {
                System.out.printf("Iteration %d: %.2f\n", iter, total);
                
                // GC情報表示
                Runtime runtime = Runtime.getRuntime();
                System.gc();  // 明示的GC
                long usedMemory = runtime.totalMemory() - runtime.freeMemory();
                System.out.printf("Memory used: %.2f MB\n", usedMemory / 1024.0 / 1024.0);
            }
        }
        
        long endTime = System.nanoTime();
        double elapsedMs = (endTime - startTime) / 1_000_000.0;
        
        System.out.printf("Total time: %.2f ms\n", elapsedMs);
    }
}