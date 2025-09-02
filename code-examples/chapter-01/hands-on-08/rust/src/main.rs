use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

// カスタムアロケータでメモリ使用量追跡
struct TrackingAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static DEALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
        }
        ret
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        DEALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
    }
}

#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;

fn get_memory_stats() -> (usize, usize) {
    (
        ALLOCATED.load(Ordering::SeqCst),
        DEALLOCATED.load(Ordering::SeqCst)
    )
}

fn memory_usage_demo() {
    println!("=== Memory Usage Analysis ===");
    
    let (start_alloc, start_dealloc) = get_memory_stats();
    
    {
        // スコープ内でメモリ使用
        let mut data = Vec::with_capacity(100_000);
        for i in 0..100_000 {
            data.push(format!("Record_{}", i));
        }
        
        let (peak_alloc, peak_dealloc) = get_memory_stats();
        let used_memory = peak_alloc - peak_dealloc;
        println!("Peak memory used: {} bytes ({:.2} MB)", 
            used_memory, used_memory as f64 / 1024.0 / 1024.0);
        
        // データ処理
        let total_length: usize = data.iter().map(|s| s.len()).sum();
        println!("Total string length: {}", total_length);
        
    } // data がスコープを抜けて自動解放
    
    let (end_alloc, end_dealloc) = get_memory_stats();
    let final_used = end_alloc - end_dealloc;
    println!("Final memory used: {} bytes", final_used);
    
    println!("Memory properly cleaned up: {}", 
        final_used <= (start_alloc - start_dealloc));
}

fn main() {
    memory_usage_demo();
}