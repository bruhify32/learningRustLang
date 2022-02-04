fn main(){
    //defining struct in rustlang
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,//Unsigned as sign in count cant be negative.
        active: bool
    }
    {//using the struct we created
        let user1 = User{
            username: String::from("Tom"),
            email: String::from("tom@g.com"),
            sign_in_count: 100,
            active: false
        };
        println!("User1 name is {} and email is {}, total login count {} and active status {}",
                            user1.username,user1.email,user1.sign_in_count,user1.active);
        //Creating Instances from Other Instances with Struct Update Syntax " .. "
        let user2 = User {
            ..user1
        };
        println!("User2 name is {} and email is {}, total login count {} and active status {}",
                            user2.username,user2.email,user2.sign_in_count,user2.active);
    }

    fn build_user(username: String,email: String) -> User {//a fn that returns struct User
        User{
            username,//we dont need to write as username field and username paramter have the same name
            email,// same as above
            sign_in_count: 1,
            active: true
        }
    }

    //Tuple Structs
    {
        //Tuple structs are useful when you want to give the whole tuple a
        //name and make the tuple be a different type from other tuples

        struct Color(i32,i32,i32);//Both Color and Point are different types as they are instance of differect sturct
        struct Point(i32,i32,i32);//even tho fields are same types.

        let black = Color(0,0,0);
        let origin = Point(0,0,0);
    }

    //Example code
    {
        #[derive(Debug)]//to able to print debugging information
        struct Rectangle{
            width: u32,
            height: u32,
        }
        let rect1 = Rectangle{ width: 30, height: 40 };
        println!("{:#?}",rect1);// can use :? or :#?
        println!("The arect of the rectangle is {}", area(&rect1));// we dont want to give ownership of rect1 to area fn.
        fn area(rectangle: &Rectangle) -> u32{
            rectangle.width * rectangle.height
        }
    }
    //Method struct
    {
        struct Rectangle{
            width: u32,
            height: u32,
        }
        let rect1 = Rectangle{ width: 30, height: 40 };
        let rect2 = Rectangle{ width: 20, height: 30};
        
        let sqr1 = Rectangle::square(50);//using associated function

        println!("The arect of the rectangle is {}", rect1.area());
        println!("Rect1 can hold rect2 {}",rect1.can_hold(&rect2));
        println!("Rect2 can hold rect1 {}", rect2.can_hold(&rect1) );

        impl Rectangle{
            fn area(&self) -> u32{
                self.width * self.height
            }
            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
            //Associated function like for example String::from()
            fn square(size: u32) -> Rectangle{
                Rectangle {width:size,height:size}
            }
        }
        //Multiple impl blocks for example like below syntaxtically correct.
        // impl Rectangle{
        //     fn area(&self) -> u32{
        //         self.width * self.height
        //     }
        // }
        // impl Rectangle{
        //     fn can_hold(&self, other: &Rectangle) -> bool {
        //         self.width > other.width && self.height > other.height
        //     }
        // }
    }


    //****************************
}