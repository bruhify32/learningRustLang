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
        let absent_number: Option<i32> = None;
    }
}