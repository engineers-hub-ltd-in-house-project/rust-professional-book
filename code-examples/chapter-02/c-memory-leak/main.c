#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

// この関数は、エラー時にメモリを解放し忘れる可能性がある
void process_data(bool error_condition) {
    // バッファを確保
    char* buffer = (char*)malloc(1024);
    if (buffer == NULL) {
        return; // 確保失敗
    }
    printf("Buffer allocated at %p\n", (void*)buffer);

    // ... bufferを使った何らかの処理 ...
    printf("Processing data...\n");

    if (error_condition) {
        printf("An error occurred! Returning early.\n");
        // エラー発生。ここでリターンしてしまうと、
        // 下の`free(buffer)`が呼ばれず、メモリリークが発生する。
        return;
    }

    // 正常終了時はメモリを解放
    printf("Processing successful. Freeing buffer.\n");
    free(buffer);
}

int main() {
    printf("--- Running normal case ---
");
    process_data(false); // この呼び出しではリークしない

    printf("\n--- Running error case ---
");
    process_data(true); // この呼び出しではメモリリークが発生する！
    // このリークを検出するには、valgrindのようなツールが必要。
    // valgrind --leak-check=full ./c-memory-leak

    printf("\nProgram finished. If run with valgrind, a leak will be reported.\n");

    return 0;
}
