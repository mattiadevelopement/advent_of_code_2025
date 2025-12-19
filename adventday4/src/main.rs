use std::fs;


fn main() {
    let contents = fs::read_to_string("real_puzzle").expect("error");
    //let silver  = find_silver(&contents);
    let silver = 0;
    let gold = find_gold(&contents);
    println!("silver: {silver}, gold: {gold}");
}


fn find_adiacent(index: [i32; 2], matrix: Vec<Vec<char>>, len_righe: usize, len_colonne: usize) -> bool {
    let rotolo: char;
    let index_colonna: usize = index[1] as usize;
    let index_linea: usize = index[0] as usize;
    let mut result = 0;
    let print  =  matrix[index_linea][index_colonna];
    //println!("{print}");
   // println!("controllando dalla posizione: {}, {}", index_linea, index_colonna);
    if matrix[index_linea][index_colonna]  != '@'{
        return false
    }
    else {for i in 1..4{
        for j in 1..4{
            if (index_linea as i32 + i as i32 - 2) < 0 || (index_colonna as i32 + j as i32- 2) < 0 {continue}
            if (index_linea as i32 + i as i32 - 2) >= len_righe as i32 || (index_colonna as i32 + j as i32- 2) >= len_colonne as i32 {continue}
            if i == 2 && j == 2 {continue}
            if matrix[index_linea + i - 2][index_colonna + j - 2] == '@'{
                result = result + 1;
                let print1 = index_linea + i - 2;
                let print2 = index_colonna + j - 2;
    //            println!("controllando posizione: {print1}, {print2}");
            }
        }
    }
}
    if result < 4{
//        println!("true");
        return true

    }
    else {return false}
}


fn find_silver(contents: &str) -> i32{
    let mut result = 0;
    let lines: Vec<&str> = contents.lines().collect();
    let mut point = [0, 0];
    let max_paper = 4;
    let mut paper: i32;
    let mut linee: Vec<_> = contents.lines().collect();
    let a = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut index_linea = 0;
    let mut index_colonna = 0;
    let mut charrr: char;
    for linea in contents.lines()  {
        let mut matrix_linea: Vec<char> = Vec::new();
        index_colonna = 0;
        for charrr in linea.chars(){
            matrix_linea.push(charrr);
            index_colonna = index_colonna + 1;
        }
        matrix.push(matrix_linea);
        index_linea = index_linea + 1;
    }
    println!("matrix is: {:?}", matrix);
    while true{
        println!("index riga: {}, index colonna: {}", index_linea, index_colonna);
         //do every line
point[0] = 0;
        for line in &lines{
           println!("line: {line}");
            //do all the line
            point[1] =  0;
            for charr in line.chars(){
                if find_adiacent(point, matrix.clone(), index_linea, index_colonna) {result = result + 1};
                point[1] = point[1] + 1;
                println!("letter: {charr}");
            }
                   point[0] = point[0] + 1;
        }

        //let len = lines[point[0] as usize].len();
        break
    }
    result
}


fn find_gold(contents: &str) -> i32{
    let mut result = 0;
    let mut taken: bool = true;
    let lines: Vec<&str> = contents.lines().collect();
    let mut point = [0, 0];
    let max_paper = 4;
    let mut paper: i32;
    let mut linee: Vec<_> = contents.lines().collect();
    let a = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut index_linea = 0;
    let mut index_colonna = 0; 
    //create all the matrix and also the number of lines and colums in it
    for linea in contents.lines()  {
        let mut matrix_linea: Vec<char> = Vec::new();
        index_colonna = 0;
        for charrr in linea.chars(){
            matrix_linea.push(charrr);
            index_colonna = index_colonna + 1;
        }
        matrix.push(matrix_linea);
        index_linea = index_linea + 1;
    }
    //end matrix creation
  //  println!("matrix is: {:?}", matrix);
    while taken{
        taken =  false;
    //    println!("index riga: {}, index colonna: {}", index_linea, index_colonna);
         //do every lines
point[0] = 0;
        for line in &lines{
      //     println!("line: {line}");
            //do all the line
            point[1] =  0;
            
            for charr in line.chars(){
                if find_adiacent(point, matrix.clone(), index_linea, index_colonna) {result = result + 1; matrix[point[0] as usize][point[1] as usize] = '.'; taken = true;};
                point[1] = point[1] + 1;
        //        println!("letter: {charr}");
            }
                   point[0] = point[0] + 1;
        }

        //let len = lines[point[0] as usize].len();
    //    println!{"{:?}", matrix};
    }
    result
    
}