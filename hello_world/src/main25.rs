//Define Traits
trait Operation<T>{
    fn add(&mut self, a:T, b:T);
    fn subs(&mut self,  a:T, b:T);
}

//Implement Operation to certain data types
impl Operation<u32> for u32 {
    fn add(&mut self, a:u32, b:u32){
      *self =  a + b;
    }

    fn subs(&mut self, a:u32, b:u32){
      *self =  a - b;
    }
}

impl Operation<f64> for f64 {
    fn add(&mut self, a:f64, b:f64){
        *self = a+b
    }

    fn subs(&mut self, a:f64, b:f64){
        *self = a-b;
    }
}

fn main(){
    let mut u32_number: u32 = 0;
    u32_number.add(1,1);
    println!("{}", u32_number);
    u32_number.subs(1,1);
    println!("{}", u32_number);


    let mut f64_number: f64 = 0.0;
    f64_number.add(1.0 ,1.0 );
    println!("{:.2}", f64_number);
    f64_number.subs(1.0  ,0.8);
    println!("{:.2}", f64_number);

   
}