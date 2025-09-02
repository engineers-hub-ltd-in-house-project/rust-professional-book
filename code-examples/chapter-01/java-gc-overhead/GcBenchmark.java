import java.util.ArrayList;
import java.util.List;

public class GcBenchmark {
    public static void main(String[] args) {
        System.out.println("Starting GC benchmark...");
        long startTime = System.nanoTime();

        // このループは、ガベージコレクタに負荷をかけるために、
        // 多数の短命なオブジェクトを生成するように設計されています。
        for (int i = 0; i < 1000; i++) {
            List<String> tempList = new ArrayList<>();
            for (int j = 0; j < 10000; j++) {
                // ループ内で新しいStringオブジェクトを作成
                tempList.add(new String("Object " + j));
            }
            // ここでtempListがスコープを抜けるため、オブジェクトがGCの対象となる
        }

        long endTime = System.nanoTime();
        System.out.printf("Total time: %.2f ms\n", (endTime - startTime) / 1_000_000.0);
        System.out.println("Run with -Xlog:gc* to observe GC activity.");
    }
}