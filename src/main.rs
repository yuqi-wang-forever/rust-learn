#![allow(unused)]
use std::io;
use rand::Rng;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add; //允许泛型进行数学运算操作
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc,Mutex};
// 引入自己的module
mod tvshows;
use crate::tvshows::watch_stargate;
// cargo new 项目名
// 第一次运行或者build项目会生成lock文件
fn main() {
   thread_join();
}
fn bank_solve() {
    pub struct Bank{
        balance: f32,
    }
    fn withdraw(the_bank:&Arc<Mutex<Bank>>, amt: f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.0 {
            println!("Current balance:{} withdrawal a smaller amount",bank_ref.balance);

        } else {
            bank_ref.balance -= amt;
            println!("Current balance:{} after customer withdrawal {}",bank_ref.balance,amt);
        }
    }
    fn customer(the_bank:Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.0);
    }
    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank {balance: 20.0}));
    let handles= (0..10).map(|_|{
        let bank_ref: Arc<Mutex<Bank>> = bank.clone();
        thread::spawn(||{
            customer(bank_ref)
        })
    });
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Total {}",bank.lock().unwrap().balance);

}
fn bank() {
    pub struct Bank{
        balance: f32,
    }
    // fn withdraw(the_bank: &mut Bank,amt: f32) {
    //     the_bank.balance -= amt;
    // }
    // let mut bank = Bank{balance: 100.0};
    // withdraw(&mut bank, 5.0);
    // println!("Banlance:{}",bank.balance);
    // fn customer(the_bank: &mut Bank,amt: f32) {
    //     withdraw(the_bank, amt);
    // }
    // thread::spawn(||{ //会出现问题
    //     customer(&mut bank, 5.0)
    // }).join().unwrap();
}
fn thread_join() {
    let thread1 = thread::spawn(||{
        for i in 1..25 {
            println!("Spawn thread:{}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..20 {
        println!("main thread:{}",i);
        thread::sleep(Duration::from_millis(1));
    }
    thread1.join().unwrap();
}
fn concurrency() { // 最多打印19个 需要使用join操作将任务完成
    let thread1 = thread::spawn(||{
        for i in 1..25 {
            println!("Spawn thread:{}",1);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..20 {
        println!("main thread:{}",i);
        thread::sleep(Duration::from_millis(1));
    }
}
// 二叉树的数据结构
fn binary_tree() {
    struct TreeNode<T>{
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }
    impl<T> TreeNode<T> {
        pub fn new(key:T) -> Self {
            TreeNode { left: None, right: None, key: key}
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self,node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }
    let node1 = TreeNode::new(1)
    .left(TreeNode::new(2))
    .right(TreeNode::new(3));
}
fn box_store() {
    // box:智能指针 heap上 当存储数据较多的时候使用box存储
    let b_int1 = Box::new(10);
    println!("b_int1 = {b_int1}");;
}
fn closures() {
    // let var_name = |parameters| -> return_type {body} 语法、、
    let can_vote = |age: i32| {
        age >= 18
    };
    println!("can vote: {}",can_vote(18));
    let mut samp1 = 5;
    let print_var = || println!("samp1 is :{}",samp1);
    print_var();
    samp1 = 10;
    let mut change_var = || samp1 +=1;
    change_var();
    println!("samp1 changed:{}",samp1);
    samp1 = 10;
    println!("samp1 reset:{}",samp1);
    //
    fn use_func<T>(a: i32,b: i32 ,func: T) -> i32 
    where T:Fn(i32,i32) -> i32 {
        func(a,b)
    }
    let sum = |a: i32,b: i32| a+b;
    let product = |a: i32,b: i32| a*b;
    println!("5 + 4 = {}",use_func(5,4,sum));
    println!("5 * 4 = {}",use_func(5,4,product));
    println!("5 + 4 = {}",sum(5,4));
    println!("5 * 4 = {}",product(5,4));
}
fn iterators() {
    let mut arr_it = [1,2,3,4];
    for val in arr_it.iter() {
        println!("{}",val); //无法改变值
    }
    // arr_it.into_inter(); // 集合不能再使用,可以修改值
    let mut iter1 = arr_it.iter();
    println!("1st is {:?}",iter1.next());
}
fn file() {
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
       Ok(file) => file,
       Err(error)=> panic!("problem creating file : {:?}",error),
    };
    write!(output,"Just some \n Random Words").expect("has an error when write some text");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}",line.unwrap());
    }
    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(filecreated) => filecreated,
                Err(err) => panic!("can not creaet file:{:?}",err),
            }
            _other_error => panic!("still has other error :{:?}",error),
        }
    };
}
fn error_handle() {
    panic!("this will print a error message");
}
fn mod_watch_stargate() {
    watch_stargate();
}
fn trait_struct() {
    const PI: f32 = 3.1415926;
    trait Shape { //类似面向对象中的接口
        fn new(length: f32,width: f32) -> Self;
        fn area(&self) -> f32;
    }
    struct Rectangle {length: f32,width: f32};
    struct Circle {length: f32,width: f32};
    impl Shape for Rectangle {
        // 必须要有new函数
        fn new(length: f32,width: f32) -> Rectangle {
            return Rectangle { length, width };
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }
    impl Shape for Circle {
        fn new(length: f32,width: f32) -> Circle {
            return Circle { length, width };
        }
        fn area(&self) -> f32 {
            return (self.length/2.0).powf(2.0) * PI;
        }
    }
    let rec:Rectangle = Shape::new(10.3,5.3);
    let circ:Circle = Shape::new(10.2, 10.6);

    println!("Rec area: {}",rec.area());
    println!("Circ area: {}",Circle::area(&circ));
}
fn struct_generics() {
    struct Rectangle<T,U> {
        length:T,
        height:U,
    }
    let rec = Rectangle {
        length:10,
        height:9.3,
    };
}
fn structs() {
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }
    let mut customer_jack = Customer {
        name: String::from("Jack"),
        address: String::from("SG1"),
        balance: 4000.30,
    };
    println!("{}",customer_jack.balance);
    // 修改
    customer_jack.balance = 5000.39;

    println!("{}",customer_jack.balance);
}
fn hashmaps() {
    let mut heros = HashMap::new();
    heros.insert("superman","Clark");
    heros.insert("batman","Bruce");
    heros.insert("The Flash","Barry");
    for (k,v) in heros.iter(){
        println!("{} : {}",k,v);
    }
    println!("length:{}",heros.len());
    if heros.contains_key(&"batman"){
        let the_batman = heros.get(&"batman");
        match the_batman {
            Some(x) => println!("{x} is a hero"),
            None => println!("Oops is not a hero"),
        }
    }
}
fn owner(){
    let str1 = String::from("world");
    let str2 = str1; //这样会将内存分配给str2，str1不再指向内存地址
    let str3 = String::from("world");
    let str4 = str3.clone(); // 将str3拷贝一份分给str4
    print_str(str3);
    let str_return =  print_return_string(str4);
    println!("return_string = {str_return}");
    let mut str5 = String::from("Stargate");
    change_string(&mut str5);
}
fn change_string(x: &mut String) {
    x.push_str(" Is Good");
    println!("changed x_string: {x}");
}
fn print_return_string(x: String) -> String {
    println!("A String and Return :{x}");
    x
}
fn print_str(x: String){
    println!("A String:{x}");
}

fn generics() {
    println!("5 + 6 = {}",get_sum_generics(5,6));
    println!("5.6 + 6.1 = {}",get_sum_generics(5.1,6.1));
}
fn get_sum_generics<T:Add<Output = T>>(x: T,y: T) -> T {
    x+y
}
fn sum_list_impl() {
    let num_list = vec![1,2,3,4,5,6,7];
    println!("Sum of list = {}",sum_list(&num_list));
}
fn sum_list(list:&[i32]) -> i32{
    let mut sum: i32 = 0;
    for &val in list.iter(){
        sum += &val;
    }
    sum // return省略时这样写
}
fn get_sum_return_many(x: i32) -> (i32,i32){
    
    return (x+1,x+2);
}
fn get_sum_return(x: i32,y: i32) -> i32{
    
    return (x+y);
}
fn get_sum(x: i32,y: i32){
    println!("{} + {} = {}",x,y,x+y);
    
}
fn vectors() {
    // 向量基本上和数组一样，不过只能存储相同类型的值
    let vect1: Vec<i32> = Vec::new(); //不可改没有mut
    let mut vect2 = vec![1,2,3,4];
    vect2.push(5);
    //println!("vect1:{}",vect1[0]);//数组越界
    
    println!("1st :{}",vect2[0]);
    let second_val: &i32 = &vect2[1]; //将vect2的引用给second
    println!("vect2: 2nd = {second_val}");
    match vect2.get(1) {
        Some(second) => println!("2nd is :{}",second),
        None => println!("No 2nd value"),
    }
    for i in &mut vect2 {
        *i *= 2;
        println!("{}",i);
    }
    for i in &vect2{
        println!("i:{}",i);
    }
    println!("Vect2 length:{}",vect2.len());
    println!("Pop: {:?}",vect2.pop()); //只能这样写 
}
fn enum_learn() {
    enum Day{
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }
    impl Day{
        fn is_weekend(&self) -> bool{
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }
    let today:Day = Day::Friday;
    match today {
        Day::Monday => println!("Bad Day"),
        Day::Tuesday => println!("Tuesday Day"),
        Day::Wednesday => println!("Wednesday Day"),
        Day::Thursday => println!("Thursday Day"),
        Day::Friday => println!("Friday Day"),
        Day::Saturday => println!("Saturday Day"),
        Day::Sunday => println!("Sunday Day"),
    
    }
    println!("Is today the weekend? {}",today.is_weekend());
    println!("Is today the weekend? {}",Day::is_weekend(&today));
}
fn casting() {
   let int_u8: u8 = 5;
   let int2_u8: u8 = 6;
   let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
   let int4_u64:u64 = int2_u8 as u64;
}
fn string_no_repeat() {
    let str3 = String::from("x z r t t u y i [ o p");
    let mut v1:Vec<char> = str3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}",char);
    }
    let str4 = "Random String";
    let mut str5 = str4.to_string();
    println!("{}",str5);
    let byte_arr1 = str5.as_bytes();
    
    let str6 = &str5[0..6];
    println!("string length:{}",str6.len());
    str5.clear();
    println!("str5.clear():{}",str5);
    let str6 = String::from("Just some");
    let str7 = String::from("  Just Words");
    println!("str6:{str6}");
    // let str9 = &str6 + &str7; 两边都不能是&引用，左边必须是string类型

    let str8 = str6+&str7;
    // 此处str6不再存在
    println!("str7:{str7}");
    println!("str8:{str8}");
    for char in str8.as_bytes(){
        println!("{char}"); // assic码
    }
    for char in str8.bytes(){
        println!("{char}"); // assic码
    }
}
fn string_assign() {
    let mut str1 = String::new();
    str1.push('A');
    str1.push_str(" Word");
    println!("{}",str1);
    for word in str1.split_whitespace(){
        println!("{}",word);
    }
    let str2 = str1.replace('A',"Other");
    println!("{}",str2);
    
}
fn tuple() {
    let my_tuple:(u8,String,f32) = (24,"yuqi".to_string(),5_000_000.00);
    println!("my_tuple: {}",my_tuple.1);
    println!("my_tuple: {}",my_tuple.2);
    println!("my_tuple: {}",my_tuple.0);
    println!("my_tuple: {}",my_tuple.1); // 不能使用my_tuple[1]取值
    let (v1,v2,v3) = my_tuple;
    println!("v1:{v1}");
    println!("v2:{v2}");
    println!("v3:{v3}");
}
fn array_for() {
     let arr_2 = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx = 0;
    for val in arr_2.iter(){
        println!("val: {}",val);
    }
}
fn array_while(){
    let arr_2 = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx = 0;
    while loop_idx < arr_2.len() {
        println!("arr: {}",arr_2[loop_idx]);
        loop_idx += 1;
    }
}
fn array_loop(){
    let arr_2 = [1,2,3,4,5,6,7,8,9];
    println!("length: {}",arr_2.len());
    // 遍历
    let mut loop_idx = 0;
    // 打印出所有奇数
    loop {
        if arr_2[loop_idx] %2 == 0{
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("val : {}",arr_2[loop_idx]);
        loop_idx += 1;
    }
}
fn array(){
    let arr_1 = [1,2,3,4];
    
    println!("1st: {}",arr_1[0]);
    
}
fn match_demo2(){
    let age = 24;
    let marry_age = 22;
    match age.cmp(&marry_age) { //cmp要把大于，小于，等于全部写上
        Ordering::Less => println!("Oops you can not get married"),
        Ordering::Greater =>println!("nice you could do it"),
        Ordering::Equal =>println!("you could do it,also"),
    }
}
fn match_demo1(){
    let age2 = 8;
    match age2 {
        1..=18 => println!("you are a adult"),
        30 | 100 => println!("you are a wonderful human being"),
        50..=i32::MAX => println!("you are a so good person"),
        _ => println!("all is well"), // 没有分号
    };
}
// rust 的三元运算符与其他语言不一样，只能使用if实现类似的效果
fn ternary_operator(){
    let mut age: u32 = 24;
    let can_marry = if age >= 22 {
        true
    } else {
        false
    };
    
    println!("can marry:{can_marry}");
}
fn if_else(){
    let age: u32 = 24;
    if (age > 0)&&(age < 18){
        println!("youngth");
    } else if (age == 30 || age ==100) {
        println!("good age")
    } else if age > 50 {
        println!("maybe little yried");
    } else {
        println!("always good");
    }
}
fn randm(){
    let random_num = rand::thread_rng()
        .gen_range(1..101);
}
// u32的基础运算，rust没有内置++运算符
fn math_u32(){
    
    let mut num_3:u32 = 5;
    let num_4: u32 = 4;
    println!("5+4 = {}",num_3+num_4);
    println!("5-4 = {}",num_3-num_4);
    println!("5*4 = {}",num_3*num_4);
    println!("5/4 = {}",num_3/num_4);
    println!("5%4 = {}",num_3%num_4);
    num_3 += 1;
    println!("{num_3}");
}
// float精度问题
fn math_f32_f64(){
    // float 精度
    let num_1:f32 = 1.1111111111111111111111;
    // f32有六位精度
    println!("f32: {}",num_1+0.1111111111111111111111);

    // f64有14位精度
    let num_2:f64 = 1.1111111111111111111111;
    println!("f64: {}",num_2+0.1111111111111111111111);
}
fn data_type(){
    // 查看各种类型的最大值
    println!("max u32:{}",u32::MAX);
    println!("max u64:{}",u64::MAX);
    println!("max u128:{}",u128::MAX);
    println!("max usize:{}",usize::MAX);
    println!("max f32:{}",f32::MAX);
    println!("max f64:{}",f64::MAX);
    println!("max i32:{}",i32::MAX);
    println!("max i64:{}",i64::MAX);
    println!("max isize:{}",isize::MAX);
    // boolean
    let is_true = true;
    let is_false = false;
    // char
    let my_grade = 'A';
    
}
fn const_shadowing(){
    const ONE_MIL: u32 = 1_000_00; 
    const PI:f32 = 3.14;
    let age = "47";
    let mut age:u32 = age.trim().parse() // 将string类型转换为u32且可变
        .expect("age wasn't assigned a number");
    age = age + 1;
    println!("I am {} and I want ${}",age,ONE_MIL);
}
// 变量，常量，随机数和命令行读取
fn val_io_read_line_random(){
    let mut name = String::new(); // let mut代表声明一个可变的变量
    let num = 5; // 声明一个不可变的常量
    
    println!("What is your name?");
    io::stdin()
        .read_line(&mut name) //读取命令行的参数
        .expect("Failed to read line"); //若失败打印出信息
    println!("Hello,{name}"); // {}:表示读取 将命令行中写的参数写入，name的值并打印到终端
    println!("Hello,{}",name); // 两种方式
    let secret_number = rand::thread_rng().gen_range(1..=100); //获取1-100以内的随机值
    println!("The secret number is : {secret_number}");
}