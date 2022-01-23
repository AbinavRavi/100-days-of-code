struct Calculator{
    num1:u32,
    num2:u32,
}

impl Calculator{
    fn add(&self) -> u32{
        let add = self.num1 + self.num2;
        return add;
    }

    fn subtract(&self) -> u32{
        let sub;
        if self.num1 > self.num2{
            sub = self.num1 - self.num2;
        }
        else{
            sub = self.num2 -self.num1;
        }
        return sub;
    }

    fn multiply(&self) -> u32 {
        let mult;
        mult = self.num1 *self.num2;
        return  mult;

    }


}

fn main() {
    let nums = Calculator{
        num1: 10,
        num2: 20,
    };

    print!("The calculator values are {},{},{}",nums.add(),nums.subtract(),nums.multiply());
}
