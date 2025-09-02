#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    int id;
    char *name;     // 動的割り当て
    double value;
} Record;

// 動的メモリ管理版
Record* create_records(int count) {
    Record *records = malloc(count * sizeof(Record));
    if (!records) return NULL;
    
    for (int i = 0; i < count; i++) {
        records[i].id = i;
        
        // 文字列の動的割り当て
        records[i].name = malloc(50);
        snprintf(records[i].name, 50, "Record_%d", i);
        
        records[i].value = i * 1.5;
    }
    
    return records;
}

void free_records(Record *records, int count) {
    for (int i = 0; i < count; i++) {
        free(records[i].name);  // 忘れがち！
    }
    free(records);
}

// バグを仕込んだ版（教育用）
void buggy_process() {
    Record *records = create_records(1000);
    
    // 処理
    double total = 0.0;
    for (int i = 0; i < 1000; i++) {
        total += records[i].value;
    }
    
    printf("Dynamic approach total: %.2f\n", total);
    
    // BUG: メモリリーク！free_records を呼び忘れ
    // free_records(records, 1000);
}

int main() {
    buggy_process();
    return 0;
}