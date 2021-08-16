#include <stdio.h>

int main()
{
    void* rs = RustStruct_create();
    RustStruct_show(rs);
    RustStruct_destroy(rs);
}