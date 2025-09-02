#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    int id;
    char *name;
    double value;
} Record;

Record* create_records(int count) {
    Record *records = malloc(count * sizeof(Record));
    if (!records) return NULL;
    
    for (int i = 0; i < count; i++) {
        records[i].id = i;
        records[i].name = malloc(50);
        if (records[i].name) {
            snprintf(records[i].name, 50, "Record_%d", i);
        }
        records[i].value = i * 1.5;
    }
    return records;
}

void free_records(Record *records, int count) {
    for (int i = 0; i < count; i++) {
        free(records[i].name); // 内側のポインタの解放を忘れがち
    }
    free(records);
}

int main() {
    Record *records = create_records(1000);
    // ... 何らかの処理 ...
    printf("Records created. Intentionally leaking memory by not calling free_records.\n");
    // BUG: free_records(records, 1000); を呼び忘れる
    return 0;
}
