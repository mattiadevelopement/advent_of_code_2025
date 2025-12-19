use std::fs;
fn main() {
    let contents = fs::read_to_string("real_puzzle").expect("ll");
    let splitt: Vec<_> = contents.split("\r\n").collect();
    let mut silver = 0;
    println!("{:?}", splitt);
    let mut ranges: Vec<&str> = Vec::new();
    let mut ingredients: Vec<i64> = Vec::new();
    let mut gold = 0;
    let mut separator_not_found = true;
    for i in splitt{
        if i.eq("") {separator_not_found = false; continue}
        if separator_not_found{ranges.push(i);}
        else {ingredients.push(i.parse::<i64>().expect("err"))}
    } 
      //  println!("{:?}", ingredients);

/*    println!("{:?}", ranges);
    for ingredient in ingredients{
        if find_silver(ingredient, ranges.clone()){silver = silver + 1;}
    }
  */  gold = find_gold(ranges.clone());
   println!("silver: {silver}, gold: {gold}");

}
fn find_silver(ingredient: i64, ranges: Vec<&str>) -> bool{
    let mut is_in_ranges = false; 
 //   println!("working with ingredient: {ingredient}");
    for range in ranges{

        let range: Vec<_> = range.split("-").collect();
        if ingredient > range[0].parse::<i64>().expect("a") && ingredient < range[1].parse::<i64>().expect("a")+ 1{
            is_in_ranges =  true;
        }
        //println!("{:?}", range);
    }
    is_in_ranges
}

fn find_gold(ranges: Vec<&str>) -> i64{
    let mut result = 0;
    let mut matrix:  Vec<Vec<i64>> = Vec::new();
    for range in ranges{
        let range: Vec<_> = range.split("-").collect();
        let range0 = range[0].parse::<i64>().expect("aa");
        let range1 = range[1].parse::<i64>().expect("aa") + 1;
        matrix.push(vec![range0, range1]);
    }
    let mut new_matrix:  Vec<Vec<i64>> = Vec::new();
    let mut len = 0;
    let mut number_to_skip: Vec<usize> = Vec::new(); 
    let mut range0 = 0;
    let mut range1 = 0;
    for range in &matrix{
        range0 = range[0];
        range1 = range[1];
        let mut temporary_len = len + 1;
        for range in &matrix[temporary_len..]{
            let test_range0 = range[0];
            let test_range1 = range[1];
            //if range0 == test_range0 && range1 == test_range1 {println!("insideeeeeeeee {range0} {test_range0}")}                                                                                                                                // a   a2   b        b2?
            if range0 <= test_range0 && range1 > test_range0{       //  --|---|----|-------             
                                                                                                    // a   a2     b2   b               
                if range1 >= test_range1{
                    println!("innside test1 - 1 {} - {} in {} - {} and skipping index {}", range0,  range1 - 1, test_range0, test_range1 - 1, temporary_len);
                    number_to_skip.push(temporary_len)}                      //  --|---|----|----|--     
                else {
                    println!("innside test1 - 2 {} - {} in {} - {}", range0,  range1 - 1, test_range0, test_range1 - 1);
                    range1 = test_range0;}
            }   
            if range0 == test_range0 && range1 == test_range1{number_to_skip.push(temporary_len); println!("skipping index {}, len is: {len}, numers are {range0} and {range1}", temporary_len);continue;}
           // if range0 > test_range0 &&  range1 < test_range1 {println!("innside test2 {}", range0)}
            if range0 > test_range0 && range0 < test_range1 {
                if range1 >= test_range1 {
                    println!("innside test3-1 {} - {} in {} - {}", range0,  range1 - 1, test_range0, test_range1 - 1);
                    range0 = test_range1}
                else {
                    println!("innside test3-2 {} - {} in {} - {}, len is {}  temporary len is {}", range0,  range1 - 1, test_range0, test_range1 - 1, len, temporary_len);
                    number_to_skip.push(len); println!("{range0}, {range1}, {test_range0}, {test_range1}");}
            };
            
            temporary_len = temporary_len + 1;
           // if range1 < test_range0 && range0 > test_range1 {continue}
            //if range1 < test_range0 && range1 <  test_range1 {println!("innside{}", range0);}
        }
      //  println!("range1 herrre is {}", range1);
        new_matrix.push(vec!(range0, range1));
//        if range1 - range0 >  0{
//            result = result + range1 - range0 
//        }        
       len = len + 1;
    }
 //   println!("{:?}", &matrix);
    
    println!("{:?}", &new_matrix);
    println!("{:?}", &number_to_skip);
    let mut index = 0;
    for range in &new_matrix{
        range0 = new_matrix[index][0]; 
        range1 = new_matrix[index][1];
        let sum = range1 - range0; 
        if number_to_skip.contains(&(index as usize)){

        }
        else {
        println!("summing {} and {}  and index is {index} and sum is {}", range0, range1, sum);
        result = result + range1 - range0;}
        index = index + 1;
    }
    result
}

