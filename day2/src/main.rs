use std::fs;


fn main() {
    let contents = fs::read_to_string("src\\ids").expect("error");
    let ranges: Vec<&str> = contents.split(",").collect();
    let range: &str;
    let mut result: i64 = 0;
    let mut b: Vec<&str>;
    let mut first_number;
    let mut second_number;
    let mut c;
    let mut len;
    let mut index: i64;
    let mut point: usize = 0;
    let mut midpoint: usize;
    for a in &ranges{
      //  println!("llll {:?}", a);
        b = a.split("-").collect();
        //println!("Hello, world! {}", {a});
        first_number =  b.get(0).unwrap().parse::<i64>().expect("stupide error");
        second_number =  b.get(1).unwrap().parse::<i64>().expect("stupid error");
        index = first_number;
        println!("{}",{index});
        while index <= second_number{
            c = index.to_string();
            len = (c.len() / 2) + 1;
//println!("the len is: {}", len);
            point = 0;
            let mut digit_sequences = c.chars();
            for i in 1..len{
                
                let mut repetiton = control_digits_repetition(&c, &c[0..i]);
                ("{repetiton}");
                if repetiton{result = result + index;/*println!("{}",index)*/;break;}
            } 
            index = index + 1;
        }
      //  println!("first: {}", {first_number});
      println!("{}",{result});
    }
    
}
fn control_digits_repetition(digits: &String, sequence: &str ) -> bool{
    // return if a set of char or a single char is repeated in all the String(digits)
   //println!("{}",{sequence});
   //println!("working with the number  {} in the sequencce {}", digits, sequence);
    let lenn =sequence.len();
    let digits_len = digits.len();
    let mut returrn = false;
   /* for i in digits.chars()
        .collect::<Vec<char>>() // Collect chars into a vector
        .chunks(2) // Create chunks of 2 characters
        .map(|chunk| chunk.iter().collect::<String>()) // Convert chunks
        {if sequence.eq(&i){
            returrn = true;
            println!("{} is equal to {}", {sequence}, {i});
        }
        }  
        returrn*/
    //    println!("{sequence}");
    let chunks: Vec<String> = digits
        .chars() // Use the chars() method to get an iterator over the characters
        .collect::<Vec<char>>() // Collect into a vector
        .chunks(lenn) // Create chunks of 2 characters
        .map(|chunk| chunk.iter().collect()) // Convert each chunk to String
        .collect(); // Collect into a vector
        let mut equals = false;
        let mut times_equals: usize = 0;
        
       // if sequence.eq("2121212121"){println!("arrivatooo");}
        for i in 0..(digits_len / lenn){
//if &chunks[i]=="21"{println!("aaa {sequence}, ");}
        if sequence.eq(&chunks[i]){times_equals= times_equals + 1}
//println!("{}", chunks[i]);
                }
        if times_equals == (digits_len / lenn) && (digits_len % lenn == 0){equals = true}    //println!("ha");}
      //  println!("the result is: {}, {}, {}",digits_len, lenn, times_equals);
        
      //  println!("the result is: {}", equals);
        equals
 }
