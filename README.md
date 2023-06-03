# rust-learn
学习rust
**windows需要安装visual studio中的c++桌面开发环境，否则会出现link.exe找不到的错误**
## 堆栈
    stack：先进后出，后进先出，数据在stack中必须定义一个确定的大小
    heap： 把数据放进堆中的时候你需要请求一定数量的空间，操作系统会找到可用的空间并返回这个空间的地址，这个内存空间的引用叫做指针
    **规则**
    - 每个值有一个叫做owner的变量
    - 在同一时间只能有一个owner
    - 当owner超出范围时变量值会消失
    rust是一个自动管理内存的语言
 ## 
 **Creates**   
    Modules that produce a library or executable
**Modules**
    Organize and handle privacy
**Package**
    Build,test and share creates
**Path**
    A way of naming an item such as a struct , function
**自写的，mod必须是mode.rs**
## Result
    Result 有两个变体（varients) Ok 和 Err
    enum Result<T,E> {
        Ok(T),
        Err(E),
    }
    T代表返回值的类型，E代表错误类型

教程源码：https://github.com/derekbanas/Rust-Tutorial
youtube:https://www.youtube.com/watch?v=ygL_xcavzQ4&t=0s