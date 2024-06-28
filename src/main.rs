use csv;

fn csv_file_reader(path:&str)->bool{
    
    let csv_res=csv::Reader::from_path(path);

    match csv_res {
        Ok(mut reader) =>{
            for res in reader.records(){
                match res {
                    Ok(rec) => {
                        println!("{:?}",rec);
                    },
                    Err(e) => {
                        eprintln!("{:?}",e);
                        return false;
                    }
                }
            };

            return true;
        },
        Err(e) => {
            eprintln!("{}",e);
            return false;
        }
    }
        
}

fn main(){
    println!("\nWelcome to the CSV File Reader Tool!");
    println!("🦀🦀 🦀🦀🦀 🦀🦀🦀🦀🦀 🦀🦀🦀 🦀🦀\n");
  
    let path="./test.csv";


   let result=csv_file_reader(path);

   if result {
    println!("\nCSV file read successfully");
   }else{
    eprintln!("\nFailed to read csv file!!");
   }

}