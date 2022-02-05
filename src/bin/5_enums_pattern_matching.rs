fn main(){
    
    {//Defining enum and using the enum
        enum IpAddrKind{
            V4,
            V6,
        }
        let four = IpAddrKind::V4;// both four and six are of the same type with V4 and V6 values
        let six = IpAddrKind::V6;

        //fn that takes any IpAddrKind 
        fn routes(ip_kind: IpAddrKind){}
        routes(four);
        routes(six);
    }
    {
        enum IpAddrKind{
            V4(u8,u8,u8,u8),
            V6(String),
        }
        let home = IpAddrKind::V4(127,0,0,1);
        let loopback = IpAddrKind::V6(String::from("::1"));
    }
    {
        enum Message{
            Quit,//no data associated with it
            Move {x:i32,y:i32},//includes anonymous struct in it 
            Write(String),
            ChangeColor(i32,i32,i32),
        }
        //as like struct we can impl on enum
        impl Message{
            fn call(&self){
                // Todo
            }
        }
        let m = Message::Write(String::from("Hello"));
        m.call();
    }
    // option enum
    {
        //Defining option enum
        enum Option<T>{
            Some(T),// for now <T> means the Some variant of the Option
            None,
        }
        let some_number = Some(5);
        let some_string = Some("A String");
        // let absent_number: Option<i32> = None;
    }
    //The match control flow operator
    {
        #[derive(Debug)]
        enum UsStates{
            Alabama,
            Alaska
        }
        enum Coin{
            Penny,
            Nickel,
            Dime,
            Quarter(UsStates),
        }
        fn values_in_cents(coin: Coin) -> u8 {
            match coin{
                Coin::Penny => {
                    println!("Lucky penny");
                    1
                },
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}",state);
                    25
                },
            }
        }
        println!("{}",values_in_cents(Coin::Penny));
        println!("{}",values_in_cents(Coin::Quarter(UsStates::Alaska)));
    }
    //Matching with Option<T>
    {
        #[derive(Debug)]
        enum Option<T>{
            Some(T),
            None,
        }
        fn plus_one(x: Option<i32>) -> Option<i32>{
            match x {
                Option::Some(i) => Option::Some(i+1),
                Option::None => Option::None,
            }
        }
        let five = Option::Some(5);
        let six = plus_one(five);
        println!("{:?}",six);

        // the  _Placeholder
        let some_u8_value = 0u8;
        match some_u8_value {
            1 => println!("one"),
            3 => println!("Three"),
            5 => println!("Five"),
            _ => (),
        }
        
    }
}