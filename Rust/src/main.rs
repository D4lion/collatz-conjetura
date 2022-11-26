fn main (){
  let mut _v: Vec<i32> = Vec::new();
  let mut n = 27;

  
  while n > 1{
    if n%2 == 0{
      n = n/2;
      _v.push(n);
    } else{
      n = (n*3) + 1;
      _v.push(n);
    
    }
  };

  for i in &_v{
    println!("{}", i)
  }

}