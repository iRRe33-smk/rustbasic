use std::time::{Duration, Instant};

fn main() {
    let N = 1E8 as i32;
    let m  = 1000 as f64;
    let mut vec = Vec::new();

    for n in 1..N{
        vec.push(n % 10 + 1 );

    }
    //print!("{:?}",vec);

    //let cs:Vec<f64> = cumsum(vec.clone(), m);
    //println!("summa: {:?}",cs.last());
    
    
    let now = Instant::now();
    let res_rec = bin_rec_prod(vec.clone());
    let time = now.elapsed();

    println!("{} : {:#?}",res_rec,time);


    let now = Instant::now();
    let res_it = prod(vec);
    let time = now.elapsed();

    println!("{} : {:#?}",res_it,time)
}


fn cumsum(x: Vec<f64>, m:f64) -> Vec<f64> {
    
    let mut res_vec: Vec<f64>  = Vec::new();
    res_vec.push(x[0]);
    
    let n = x.len();
    for ind in 1..n{
        res_vec.push((res_vec[ind-1] + x[ind]) % m);
        
    }
    
    return res_vec;
}


fn bin_rec_prod<T>(x: Vec<T>) -> T
    where T: std::ops::Mul<Output = T> + Copy + std::fmt::Debug
    {
        // println!("{:?}",x);
    if x.len()>1{
        let pivot = ((x.len()/2) as f64).floor() as usize;
        // print!("{} ",pivot);

        let a =bin_rec_prod(x[0..pivot].to_vec());
        let b = bin_rec_prod(x[pivot..x.len()].to_vec());

        return a * b;
    }else{
        
        return x[0];
    }

}

fn prod<T>(x:Vec<T>)->T
where T: std::ops::Mul<Output = T> + Copy + std::fmt::Debug
{   let len = x.len();
    if len>1{
        let mut res = x[0].clone();
        for ind in 1..len{
            res = res *  x[ind];

        }
        return res;
    }else if   len ==1 {
        return x[0].clone()
    }else{
        panic!("vector must have at least one element!")
    }
        
    }


 


