fn main(){

    //ownership and function*******************
    {
        let s = String::from("Hello");// s comes in scope
        takes_ownership(s);//s's values moves into the function
        // println!("{}",s);// it will throw error as s is out of scope
        
        let x = 5;//x comes into scope
        makes_copy(x);//x's value copies into the function
        println!("X can be used still {}",x);// x is still in scope
        fn takes_ownership(some_string: String){
            println!("{}",some_string);
        }
        fn makes_copy(some_num: i32){
            println!("{}",some_num);
        }
    }// x and s variable are out of scope
    
    //Return values and scope
    {
        let s1 = gives_ownership();//gives ownership moves its return to s1
        let s2 = String::from("Hello");//s2 string comes in the scope
        let s3 = takes_and_gives_back(s2);//s2 moves its value into takes_and_gives_back() which also moves its return to s3
        //s2 is out of scope as it is moved into s3

        fn gives_ownership() -> String{// this fn will move the return values into the function that calls it
            let some_string = String::from("Hello");//some_string comes into scope
            some_string//some_stirng is returned and moves out to the calling
        }// some_sting is out of scope
        fn takes_and_gives_back(some_string: String) -> String{//takes a string and returns it
            some_string
        }
    }//s3 and s1 is out of scoped and is dropped 
    //*****************************************************

    //Reference and borrowing
    {
        let s1 = String::from("Hello");
        let len = calculate_length(&s1);//we borrowed the value of s1 using reference to the s1
        println!("The length of {} is {}", s1, len);
        fn calculate_length(s: &String) -> usize{
         s.len()   
        }
        //Mutable reference
        let mut s = String::from("hello");
        change(&mut s);
        println!("{}",s);
        fn change(some_String: &mut String){
            some_String.push_str(", world");
        }
    }


}

