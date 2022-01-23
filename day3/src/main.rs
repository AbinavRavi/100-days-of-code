struct Calculator{
    num1:u32,
    num2:u32,
}

impl Calculator{
    fn add(&self) -> u32{
        self.num1 + self.num2;
    }

    fn subtract(&self) -> u8{
        if self.num1 > self.num2{
            self.num1 - self.num2;
        }
        else{
            self.num2 -self.num1;
        }
    }


}

fn main() {
    println!("Hello, world!");
}
