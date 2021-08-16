
//结构体前置声明;
struct RustStruct;

//函数声明;
void RustStruct_show(struct RustStruct* rs);
struct RustStruct* RustStruct_create();
void RustStruct_destroy(struct RustStruct* rs);

int main()
{
    //创建对象;
    struct RustStruct * rs = RustStruct_create();
    //方法调用;
    RustStruct_show(rs);
    //销毁对象;
    RustStruct_destroy(rs);
    return 0;
}
