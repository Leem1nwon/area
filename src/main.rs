struct Object {
    width : u32,
    height : u32,
}

impl Object {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn new(width: u32, height: u32) -> Object {
        Object{
            width,
            height
        }
    }

    fn show(&self) {
        println!("{} x {} with area : {}", self.width, self.height, self.area());
    }
}

fn main() {
    let o:Object = Object {
        width: 10,
        height: 15,
    };
    
    let j:Object = Object {
        width : 13,
        height : 11,
    };

    let j : Object = Object::new(57, 83);

    o.show();
    j.show();

    println!("Area is {}", o.width * o.height);
    println!("Area is {}", j.width * j.height);
}

