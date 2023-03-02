use rand::Rng;
// use std::env;
// use std::cmp;




fn main() {

    // let args: Vec<String> = env::args().collect();
    // dbg!(args);

    // let default_t = 100 as usize;
    // let T = args.get(1).unwrap_or(100) as uint;
    // let n : usize = 

    // println!("{}",T);
    const T: usize = 1000 as usize;
    // const PINIT:i32 = 0;
    const PMIN: i32 = 1;
    const PMAX: i32 = 101;

    let mut rng = rand::thread_rng();
    let mut prices = vec![0;T];
    // let mut prices = [Pinit, i32::T];
    for t in 0..T {
        prices[t] = rng.gen_range(PMIN..PMAX);
    }
    // println!("{:?}",prices);
    
    fn local_best(prices:&Vec<i32>) -> ([usize;2],i32) {
        let tmax = prices.len();
        let mut days:[usize;2] = [0;2];
        let mut best_profit:i32 = -100000;

        for i in 0..tmax-1 {
            for j in i+1..tmax{

                let profit = prices[j] - prices[i];
                if profit > best_profit{
                    best_profit = profit;
                    days[0] = i;
                    days[1] = j;
                }
            }
        }
        // println!("best profit: {}, best days: {:?}",best_profit,days);
        return (days,best_profit)
    }

    fn global_best(prices:&Vec<i32>)-> ([usize;4],i32) {
        let tmax = prices.len();
        let mut days:[usize;4] = [0;4];
        let mut best_profit:i32 = -100000;

        for i in 2..T-2{
            let res1 = local_best(&prices[0..i].to_vec());
            let res2 = local_best(&prices[i..tmax].to_vec());

            let profit = res1.1 + res2.1;
            if profit > best_profit{
                best_profit = profit;
                days[0] = res1.0[0];
                days[1] = res1.0[1];
                days[2] = res2.0[0]+i;
                days[3] = res2.0[1]+i;
    
            }

        }
        return (days,best_profit)
    }

    let res_local = local_best(&prices);

    println!("best trades {:?}, profit: {}",res_local.0,res_local.1);

    let res_global = global_best(&prices);
    println!("best trades {:?}, profit: {}",res_global.0,res_global.1);
}


