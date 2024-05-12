use std::thread::sleep;
use std::time::Duration;

//loop死循环
//continue；语句表示本次循环内，后面的语句不再执行，直接进入下一轮循环。
//break: 表示跳出循环不再继续。
fn loop_test1()
{ 
    println!("******************** loop_test1 start ********************");

    let mut count = 0u32;
    loop{
        count +=1;
        if count == 5 {
            println!("###count=={},continue!",count);  
            continue;
        }
        println!("print count={}",count);    
        sleep(Duration::from_millis(100)); //睡眠 100 ms
        if count>10{
            break; //如果没有break则一直死循环。
        }
    }
    println!("******************** loop_test1 end ********************\n");
}
//loop循环 + 跳出标签所指的那层循环。
fn loop_test2()
{
    println!("******************** loop_test2 start ********************");
    
    let mut count1 = 0u32;
    let mut count2 = 0u32;
    'loop1:loop{
        count1 +=1;
        
        'loop2:loop{
            count2+=1;
            if count2 == 2 {
                println!("###count2=={},continue 'loop2!",count2);  
                continue 'loop2;
            }
            
            if count2 >= 4 {
                count2 = 0;
                break ;
            }
            
            if count1 == 5{
                println!("###count1=={},break: out of 'loop1!",count1);
                break 'loop1;
            }
            
            println!("print count1={} -- count2={}",count1,count2);
        }
    }
    println!("******************** loop_test2 end ********************\n");
}
//loop结构作为表达式的一部分
fn loop_test3()
{ 
    println!("******************** loop_test3 start ********************");

    let mut count = 0u32;
    println!("print count={}",count);

    count = loop{
        break 99;
    };
    println!("print count={}",count);
    
    #[warn(unreachable_code)]    {
        //count = loop{};//代码运行到这里会进入死循环。后面代码不再执行。
        //println!("print count={}",count);
    }

    println!("******************** loop_test3 end ********************\n\n");
}



//while语句是带条件判断的循环语句,同样可有continue和break来控制循环流程。。
fn while_test1()
{ 
    println!("******************** while_test1 start ********************");

    let mut while_count = 0;
    while while_count < 10 {
        while_count+=1;

        if while_count == 3 {
            println!("###continue: while while_count= {}",while_count);
            continue;
        }

        if while_count == 5 {
            println!("###break: out of while while_count = {}",while_count);
            break;
        }
        println!("print while_count={}",while_count);
    }
    println!("******************** while_test1 end ********************\n\n");
}
//while死循环。
fn while_test2()
{ 
    println!("******************** while_test2 start ********************");
    // let mut while_count = 0;
    // #[warn(while_true)]{
    //     while true {
    //         while_count+=1;
    //         println!("print while_count={}",while_count);
    //         sleep(Duration::from_millis(500)); //睡眠 500 ms

    //         if while_count == 10 {
    //             break; //如果没有break则一直循环。
    //         }
    //     } 
    // }
    println!("******************** while_test2 end ********************\n\n");
}
//loop和while true无限循环区别。
fn loop_while_infinitecycle_test3(){ 
    println!("******************** loop_while_infinitecycle_test3 start ********************");
    let x;
    loop { x = 1; break; }
    println!("{}", x);//没问题。

    //let x;
    //while true { x = 1; break; }
    //println!("{}", x);//报错，提示x未赋初值。

    println!("******************** loop_while_infinitecycle_test3 end ********************\n\n");
}


//for 循环。continue和break控制执行流程
fn for_test1(){ 
    println!("******************** for_test1 start ********************");

    let mut for_count: i32 ;
    for n in 1..10{
        for_count = n;
        
        if for_count == 3 {
            println!("###continue: for for_count= {}",for_count);
            continue;
        }
        if for_count == 6 {
            println!("###break: out of for for_count = {}",for_count);
            break;
        }
        println!("print for_count={}",for_count);
    }

    println!("******************** for_test1 end ********************\n");
}
//for 循环。for + match 的用法
fn for_test2(){ 
    println!("******************** for_test2 start ********************");
    
    /* 数值匹配 */
    for n in 0..=20{
        if n == 16 {
            println!("###continue: for n= {}",n);
            continue;
        }
        match n {
            0 => println!("match 0 n={}",n),
            2|4|6|8 => println!("match n is even number:{}",n),
            1|3|5|7|9 => println!("match n is odd number:{}",n),
            10..=15 => println!("match n is '10'<=n<='19' : {}",n),
            _ => println!("useless number {}",n),
        }
    }

    /* 字符串匹配 */
    let for_name = vec!["s1","s2","s3"];
    for name in for_name.iter(){
        match name{
            &"s1" => println!("match \"s1\" name ={}",name),
            &"s2" => println!("match \"s2\"  name ={}",name),
            _ => println!("current name={}, isn't = \"s1\" or \"s2\"!",name),
        }
    }

    println!("******************** for_test2 end ********************\n");
}
//for + while 循环混用。
fn for_test3(){ 
    println!("******************** for_test3 start ********************");

    'outer: for x in 1..5 {
        let mut i = 0;
        while i < x {
            i += 1;
            if i == 3 {
                println!("###break: 'outer. for: x={}, while:i={}",x,i);
                break 'outer;
            }
            println!("for: x={}, while:i={}",x,i);
        }
    }

    println!("******************** for_test3 end ********************\n\n");
}

fn main() {
    println!("Hello, lets go study cycle in rust!\n");

    /*loop*/
    loop_test1();
    loop_test2();
    loop_test3();

    /*while*/
    while_test1();
    while_test2();
    loop_while_infinitecycle_test3();
    
    /*for*/
    for_test1();
    for_test2();
    for_test3();

}




















