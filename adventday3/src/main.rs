use std::fs;

fn main() {
    let content = fs::read_to_string("puzzle.t").expect("errorrr");
    let result = find_biggest_number(&content);
    println!("{result}");
}


fn find_biggest_number(puzzle: &str) -> i64{
    let mut result  = 0;
    let mut counter = 11;
    let mut biggest_number: String =  "".to_string();
    let mut position: usize;
    for line in puzzle.lines(){
        println!("working with line: {line}");
        let mut len = line.len();
        position = 0;
        let mut biggest_number: String =  "".to_string();
        for counter in (0..12).rev()  {
            println!("{counter} and {position}" );
            let search_biggest_digitt = search_biggest_digit(&line[position..(len - counter)]);
            let a = &line[position..(len - counter)];
            println!("{a}");
            let biggest_digit = search_biggest_digitt[1];
            println!("biggest digit is: {biggest_digit}");
            //println!("{biggest_digit}");
            position =  position + (search_biggest_digitt[0] as usize);
            biggest_number.push_str(&biggest_digit.to_string());
            //println!("{biggest_number}");
        }
      /*  let search_biggest_digitt = search_biggest_digit(&line[..len - counter]);
        let biggest_digit = search_biggest_digitt[1];
        let position =  search_biggest_digitt[0];
        //println!("error is: {}",&line[position as usize..]);
        let second_biggest_digit = search_biggest_digit(&line[position as usize..]);
        //println!("position is of first biggest figit is: {}", position);
        let biggest_number: String = biggest_digit.to_string() + &second_biggest_digit[1].to_string();
       // println!("biggest number found in the line is: {}", {biggest_number});
      //  println!("{}"); */
      println!("{biggest_number}");
      result = result + biggest_number.parse::<i64>().expect("error"); 
    }
    result
}


fn search_biggest_digit(line: &str) -> [i32; 2]{
    let mut counter = 0;
    let mut  biggest_digit = 0;
    let mut position_of_digit =  0;
        //println!("line for biggest digit is: {line}");
        for digit in line.chars(){
            let digit = digit as i32 - '0' as i32;
             
            counter = counter + 1;
            if biggest_digit < digit{
                biggest_digit = digit;
                position_of_digit = counter;
            }
    }
    //println!("biggest digit found is: {biggest_digit}");
    //println!("biggest_digit: {biggest_digit}");
    [position_of_digit, biggest_digit]
    
}


