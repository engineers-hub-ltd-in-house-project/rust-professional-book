#include <stdio.h>
#include <stdlib.h>

#define MAX_RECORDS 1000  // コンパイル時に固定

typedef struct {
    int id;
    char name[50];  // 固定サイズ
    double value;
} Record;

// FORTRAN的：全て固定サイズ、スタック配置
void process_records_classic() {
    Record records[MAX_RECORDS];  // スタック上に確保
    
    // データ初期化
    for (int i = 0; i < MAX_RECORDS; i++) {
        records[i].id = i;
        snprintf(records[i].name, sizeof(records[i].name), "Record_%d", i);
        records[i].value = i * 1.5;
    }
    
    // 処理（例：合計値計算）
    double total = 0.0;
    for (int i = 0; i < MAX_RECORDS; i++) {
        total += records[i].value;
    }
    
    printf("Classic approach total: %.2f\n", total);
}

int main() {
    process_records_classic();
    return 0;
}