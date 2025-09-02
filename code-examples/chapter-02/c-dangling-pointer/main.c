#include <stdio.h>

// この関数は、解放済みのメモリ領域を指すポインタ（ダングリングポインタ）を返してしまう
int* get_dangling_pointer() {
    int local_var = 42;
    return &local_var; // `local_var`はこの関数が終了するとスタックから解放される
}

int main() {
    int* dangling_ptr = get_dangling_pointer();
    
    // `dangling_ptr`は、もはや有効ではないメモリを指している。
    // このポインタをデリファレンス（*）する行為は「未定義動作」。
    // クラッシュするかもしれないし、偶然何かが出力されるかもしれない。
    printf("Value from dangling pointer: %d\n", *dangling_ptr);
    
    return 0;
}

