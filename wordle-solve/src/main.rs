use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    struct Green {
        position :  usize,
        letter :  char
    }
    

    struct Yellow {
        position : usize,
        letter : char
    }


    struct State {
        greens :Vec<Green>,
        yellows : Vec<Yellow>,
        grays : Vec<char>,
        correct : Vec<char>,
        verbose : bool
    }
    impl State {
        fn make_guess(mut self, guess : String) -> (State){
            let charvec: Vec<char> = guess.chars().collect::<Vec<_>>();
            if self.verbose {
                println!("Incoming guess: {}",guess);
            }
                for i in 0.. charvec.len(){
                // update greens
                if self.correct[i] == charvec[i] {
                    self.greens.push(Green{position : i, letter : charvec[i]});
                    if self.verbose {
                        println!("letter:{} in position:{} is green! ",charvec[i], i );
                    }
                    //yellows
                } else if self.correct.contains(&charvec[i]) {
                        self.yellows.push(Yellow {position : i, letter : charvec[i]});
                        if self.verbose {
                            println!("letter:{} in position:{} is yellow! ",charvec[i], i );  
                        }
                    //grays
                }else{
                    self.grays.push(charvec[i]);
                    if self.verbose {
                    println!("letter:{} gray! ",charvec[i])
                    }
                }
        
            }
            return self;
        }

    }

    fn buildState(target : String) -> State {
        let mut s: State = State { 
            greens : Vec :: <Green> :: new(),
            yellows : Vec :: <Yellow> :: new(),
            grays : Vec :: <char> :: new(),
            correct : target.chars().collect::<Vec<_>>(),
            verbose : true
         };
         return s
         
    }

  

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
    }


    let filename = "words.txt";
    
    if let Ok(lines) = read_lines(filename){
        let mut words : Vec<String> = lines.into_iter().map(|x, | -> String {
            match x{
                Err(x) => "failiure".to_string(),
                Ok(x) => x
            }

        }).filter(|x| x!="failiure").collect::<>();

        

        for w in &words{
            println!("{}",w);
        }

        let target = "wispy".to_string();
        

        let mut game:State = buildState(target);

        println!("{:?}",game.correct);
        
        let mut possible_words = words;

        let startingGuess = "crane".to_string();

        

        let mut done = false;
        let mut num_it =0;
        while !done{

            if num_it == 0{
                game = game.make_guess(startingGuess.clone());
            } else{
                game = game.make_guess(possible_words[0].clone())
            }
            
            //start with green
            for g in &game.greens{
                possible_words = possible_words.into_iter().filter(|x|{

                    x.chars().nth(g.position).unwrap_or_default()==g.letter

                }).collect::<Vec<String>>();
            }
            println!("{}",possible_words.len());

            // grays
            for c in &game.grays{
                possible_words = possible_words.into_iter().filter(|x|{

                    
                    !x.contains(c.clone())

                }).collect::<Vec<String>>();
            }
            println!("{}",possible_words.len());

            //
            for y in &game.yellows{
                possible_words = possible_words.into_iter().filter(|x|{

                    x.contains(y.letter.clone())

                }).collect::<Vec<String>>();
                
            }
            num_it+=1;
            if possible_words.len()<=1 || num_it>10{
                done=true;
                println!("found the solution:{}, in {} iterations.",possible_words[0], num_it);
    
            }
            

        }

    

        

























    }
    
    
   
    
    
    // let words : Vec<String>= lines.iter().map(|x|x.as_string() ).collect();






}