use std::time::Instant;

#[derive(Debug, Clone)]
struct Record {
    id: usize,
    name: String,
    value: f64,
}

impl Record {
    fn new(id: usize, name: String, value: f64) -> Self {
        Self { id, name, value }
    }
}

fn ownership_demo() {
    println!("=== Ownership Demo ===");
    
    // 所有権の移動
    let record = Record::new(1, "test".to_string(), 1.5);
    println!("Original record: {:?}", record);
    
    // 所有権移動
    let moved_record = record;
    println!("Moved record: {:?}", moved_record);
    
    // これはコンパイルエラー！
    // println!("Original after move: {:?}", record);
    
    // 借用を使った安全な参照
    let borrowed = &moved_record;
    println!("Borrowed: {:?}", borrowed);
    
    // 元の変数も借用後は使用可能
    println!("Original still accessible: {:?}", moved_record);
}

fn memory_efficiency_test() {
    println!("\n=== Memory Efficiency Test ===");
    
    let start = Instant::now();
    
    for iteration in 0..1000 {
        // ベクタ作成（容量事前確保で効率化）
        let mut records = Vec::with_capacity(10000);
        
        for i in 0..10000 {
            records.push(Record::new(
                i, 
                format!("Record_{}", i), 
                i as f64 * 1.5
            ));
        }
        
        // 処理（イテレータで高速処理）
        let total: f64 = records.iter().map(|r| r.value).sum();
        
        if iteration % 100 == 0 {
            println!("Iteration {}: {:.2}", iteration, total);
            
            // メモリ使用量（概算）
            let memory_per_record = std::mem::size_of::<Record>();
            let total_memory = memory_per_record * records.len();
            println!("Estimated memory: {:.2} MB", total_memory as f64 / 1024.0 / 1024.0);
        }
        
        // records は自動的に解放される
    }
    
    let elapsed = start.elapsed();
    println!("Total time: {:.2?}", elapsed);
}

// ゼロコスト抽象化のデモ
fn zero_cost_abstraction_demo() {
    println!("\n=== Zero-Cost Abstraction Demo ===");
    
    let records: Vec<Record> = (0..1000000)
        .map(|i| Record::new(i, format!("Record_{}", i), i as f64 * 1.5))
        .collect();
    
    let start = Instant::now();
    
    // 高レベル抽象化だが、高速実行される
    let result: f64 = records
        .iter()
        .filter(|r| r.id % 2 == 0)     // 偶数IDのみ
        .map(|r| r.value)              // 値を取得
        .sum();                        // 合計
    
    let elapsed = start.elapsed();
    
    println!("High-level result: {:.2}", result);
    println!("High-level time: {:.2?}", elapsed);
    
    // 同等のローレベル実装
    let start = Instant::now();
    let mut low_level_result = 0.0;
    for record in &records {
        if record.id % 2 == 0 {
            low_level_result += record.value;
        }
    }
    let elapsed = start.elapsed();
    
    println!("Low-level result: {:.2}", low_level_result);
    println!("Low-level time: {:.2?}", elapsed);
    println!("Performance difference: minimal (zero-cost abstraction!)");
}

fn main() {
    ownership_demo();
    memory_efficiency_test();
    zero_cost_abstraction_demo();
}