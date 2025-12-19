use std::fs;

fn main() {
    let contents = fs::read_to_string("real_input").expect("ll");
    let lines: Vec<_> = contents.split("\r\n").collect();
    println!("{:?}", lines);
    let silver = find_silver(&lines);
    let gold = find_gold();
    println!("silver: {silver}, gold: {gold}");
}

fn parse_input<'a>(lines: &'a Vec<&'a str>) -> &'a Vec<&'a str> {
    lines
}

fn parse_inputs(lines: &Vec<&str>,) -> Vec<Vec<String>>{
    let mut vec: Vec<Vec<String>> = Vec::new();
    for i in lines {
        let mut vecnew: Vec<String> = Vec::new();
        let ffor: Vec<_> = i.split(" ").collect();
     //   println!("{:?}", ffor);
        for i in ffor {
            if i.eq("") {
                continue;
            } else {
                vecnew.push(i.to_string());
            }
        }
        vec.push(vecnew);
    }
  //  println!("{:?}", vec);
    vec
}

fn get_operation(operator: &String) -> String {
    if operator.eq("*"){
        return "multiplication".to_string();
    }
    if operator.eq("+"){
        return "addition".to_string();
    }
    String::from("null")
}

fn find_silver(lines: &Vec<&str>) -> i64 {
    let parse = parse_input(lines);
    let parsed: Vec<Vec<String>> = parse_inputs(lines);
    let mut silver = 0;
  //  println!("{:?}", parsed);
    let mut len = parsed.len() - 1;
    let mut vec_results: Vec<i64> = vec!();
    for i in 0..parsed[0].len(){
        vec_results.push(parsed[0][i].parse::<i64>().unwrap());
    }  
 //   println!("{len}");
    for i in 1..parsed.len() - 1{
        
        let mut operation = get_operation(&parsed[len][i]);
  //      println!("{operation}");
        let mut index =  0;
        for j in &parsed[i]{
 //           println!("{:?}", vec_results);
            operation = get_operation(&parsed[len][index]);
        match operation.as_str(){
            "multiplication" => {vec_results[index] = vec_results[index] * j.parse::<i64>().unwrap(); println!("additionnnn")}
            "addition" => {vec_results[index] = vec_results[index] + j.parse::<i64>().unwrap(); println!("moltiplicationnnn")}
            _ => println!("Unrecognized input."),
        }
        index =  index + 1;    
    }
//    println!("{:?}", vec_results);
      //  parsed[]
    }
    for i in 0..vec_results.len(){
        silver = silver + vec_results[i];
    }
    silver
}

fn find_gold() -> i32 {
    let mut silver = 0;
    silver
}
