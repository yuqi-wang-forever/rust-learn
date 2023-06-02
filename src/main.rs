#![allow(unused)]
use std::io;
use rand::Rng;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
// cargo new 项目名
// 第一次运行或者build项目会生成lock文件
fn main() {
   casting();
}
fn casting() {
    
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