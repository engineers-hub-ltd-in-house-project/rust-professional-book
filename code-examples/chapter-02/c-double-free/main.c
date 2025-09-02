#include <stdio.h>
#include <stdlib.h>

int main() {
    // ヒープにメモリを確保
    int* ptr = (int*)malloc(sizeof(int));
    if (ptr == NULL) {
        return 1; // メモリ確保失敗
    }
    *ptr = 10;
    printf("First allocation at %p, value is %d\n", (void*)ptr, *ptr);

    // 1回目の解放
    printf("First free.\n");
    free(ptr);

    // ... プログラムの他の部分で、多くの処理が行われる ...
    // 開発者は、ptrが既に解放されたことを忘れてしまうかもしれない

    // 2回目の解放（ダブルフリー！）
    printf("Attempting second free...\n");
    free(ptr); // 未定義動作！
    // この時点でプログラムがクラッシュする可能性が高い。
    // あるいは、メモリアロケータの内部状態が破壊され、
    // 後続のmallocで問題が発生するかもしれない。

    printf("This line will likely not be reached.\n");

    return 0;
}

