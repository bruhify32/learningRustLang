use std::io;

fn main(){
    let x = 5;//Immutable variable
    println!("{}",x);
    // x = 6;//Will give Error on immutabletype
    
    //shadowing********

    let x = 6; //Shadowing will not give error
    println!("{}",x);

    let space = "  ";
    let space = space.len();//Will work

    let mut space = "  ";//warning
    // space = space.len();// will give error as not allowed to mutate a variable type

    //*********

    //Datatypes***************

    let tup = ("Hi",1,"3");//Tupple
    println!("{}",tup.1);
    
    let list_a = [1,2,3,4];//List
    // let list_a_err = [1,2,3,4,'hi'];//Haha wont work remember rust is a strong static type
    let list_b: [i32;5] = [1,2,3,4,5]; 
    let list_c = [3;5]; // same as let list_c = [3,3,3,3,3];
    println!("{}",list_a[1]);// prints 2

    //*************** */

    //Functions
    fn one_fn(x:i32){
        println!("I am one_fn function!");
        println!("one_fn(x) parameter passed is {}",x);
    }
    fn two_call_fn(){
        println!("Just called one_fn() function from two_fn()");
        one_fn(5);//pased a integer 5 to the funtion
    }
    two_call_fn();//calling two_call_fn()

    fn three_fn_expression(){
        let x = 6;
        // let x = (let y = 6 );//Will gives you error as let y doesnt return the val
        let y = {
            let x = 3;
            x+1;
        };
        // println!("The value of y is {}",y);
    }

    fn four_fn_return_1() -> i32{
        5 //puttig semi colon at end will lead to error
    }
    fn four_fn_return_2()->i32{
        return 5;
    }
    println!("{}",four_fn_return_2());
    //****************************** 

    //Flow Control

    let num5 = 5;

    if(2<num5){
        println!("False");
    }else{
        println!("True");
    }

    let condition = true;
    let number_choose = if condition{ // it must be same type
        5
    }else{
        6
    };
    println!("{}",number_choose);
    //******************************* 

    //Loop*******************

    fn loop_1(){ //loop without condition
        loop{
            println!("I am in infinite loop stop me by Ctrl C");
        }
    }

    fn loop_2(){//Returning values from loop
        let mut counter = 0;
        let result = loop{
            counter +=1;
            if counter == 10 {
                break counter *2
            } 
        };
        println!("{}",result);
    }
    loop_2();

    fn loop_3(){ //while loop, a loop with condition
        let mut num = 5;
        while num != 0{
            println!("From fn loop_3 The number is {}",num);
            num -= 1;
        }
        println!("From fn loop_3 loop finished!");
    }
    // loop_3();

    fn loop_4(){// looping thought a collection
        let a = [1,24,23,44,5];
        for i in a.iter(){
            println!("From fn loop_4 the val of list a is {}",i);
        }
    }
    loop_4();

}