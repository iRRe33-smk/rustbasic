use std::{time::{Duration, Instant}, vec};
fn main() {
    // let N = 100 as i32;
    // let m  = 1000 as f64;
    // let mut vec = Vec::new();

    // for n in 1..N{
    //     vec.push(n % 10 + 1 );

    // }
    // //print!("{:?}",vec);

    // //let cs:Vec<f64> = cumsum(vec.clone(), m);
    // //println!("summa: {:?}",cs.last());
    
    
    // let now = Instant::now();
    // let res_rec = bin_rec_prod(vec.clone());
    // let time = now.elapsed();

    // println!("{} : {:#?}",res_rec,time);



    // let now = Instant::now();
    // let res_it = prod(vec);
    // let time = now.elapsed();

    // println!("{} : {:#?}",res_it,time)
    


    //prime sieve
    let n = 100 as i32;
    
    let mut primes = Vec::new();
    primes.push(2); //first prime
    let mut flag :bool = true;
    
    
    for i in 3..n{ // prime candidates
        println!("{}",i);
        // for p in primes{ //  current primes
            
        //     if i % p == 0{
        //         //println!("{}",i);
        //         flag = false;
        //         break
        //     }
            
        // }
        // if flag {
        //     println!("{}",i);
        //     flag = true;
        // }    
    }
    println!("{:?}",primes);

    
}





 


