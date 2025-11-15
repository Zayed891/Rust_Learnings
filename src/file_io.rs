use std :: fs;
use std:: io;

pub fn write_to_file_exercise(){
    println!("\n ---Exercise 1: Write to File---");

    let filename = "output.txt";
    let content = "Hello,Rust! \n File I/O is awesome! \n Let's learn Rust together!";

    match fs::write(filename, content){
        Ok(_) => println!("File {} written successfully", filename),
        Err(e) => print!("Error writing file : {}",e)
    }
}

pub fn read_from_file_exercise(){
    println!("\n ---Exercise 2: Read from file---");
    let filename = "output.txt" ;

    match fs::read_to_string(filename){
        Ok(contents) => {
            println!("Contents are below");
            println!("{}", contents);
        },
        Err(e) => println!("Error reading from file {}",e )
    }
}

pub fn append_to_file_exercise(){
    println!("\n ---Exercise 3: Append to file---");

    let filename = "output.txt";
    let additional_content = "\n It's been exciting to learn rust so far";

    match fs::read_to_string(filename) {
        Ok(mut contents) => {
            contents.push_str(additional_content);

            match fs::write(filename, contents) {
                Ok(_) => println!("Wrote to file : {}", filename),
                Err(e) => println!("Error writing to file {}",e)
            }
        },
        Err(e) => println!("Error reading from file {}",e)
    }
}

pub fn list_files_exercise (){
    println!("\n ---Exercise 4: List files in directory---");

    let directory = ".";

    match fs ::read_dir(directory){
        Ok(entries) =>{
            for entry in entries{
                if let Ok(entry) = entry{
                    let path = entry.path();
                    let filename = path.file_name().unwrap();

                    if path.is_file(){
                        println!("📄 {}", filename.to_string_lossy());
                    } else if path.is_dir(){
                        println!("📁 {}",filename.to_string_lossy());
                    }
;                }
            }
        },
        Err(e) => println!("Error reading from the directory {}", e)
    }
}

pub fn create_directory_exercise(){
    println!("\n ---Exercise 5: Create Directory ---");

    let dir_name = "my_data";

    match fs::create_dir(dir_name){
        Ok(_) => println!("Directory {} has been created",dir_name),
        Err(e) => {
            if e.kind()== io::ErrorKind::AlreadyExists {
                println!("Sorry! Directory {} already exists",dir_name);
            } else {
                println!("Error creating directory {}",e);
            }
        }
    }
}

pub fn file_metadata_exercise(){
    println!("\n ---Exercise 6: File metadata---");

    let filename = "output.txt";

    match fs :: metadata(filename) {
        Ok(metadata) =>{
            println!("Metadat of file {}", filename);
            println!("Size of file : {}", metadata.len());
            println!(" Readable : {}", metadata.permissions().readonly());

            if metadata.is_file() {
                println!("Type is file");
            } else if metadata.is_dir() {
                println!("Type is directory");
            }
        },
        Err(e) => println!("Error reading metadata : {}", e)
    }
}

pub fn copy_file_exercise (){
    println!("\n ---Exercise 7: Copy File---");

    let source = "output.txt";
    let destination = "output_copy.txt";

    match fs :: copy(source,destination){
        Ok(bytes_copied) =>{
            println!("File copied successfully");
            println!("Copied bytes are : {}", bytes_copied);
        },
        Err(e) => println!("Error copying file: {}",e)
    }
}

pub fn delete_file_exercise(){
    println!("\n ---Exercise 8 : Delete file---");

    let filename = "output.txt";

    match fs::remove_file(filename) {
        Ok(_) => println!("File {} deleted successfully", filename),
        Err(e) => {
            if e.kind() == io::ErrorKind:: NotFound {
                println!("File {} not found",filename);
            }else {
                println!("Error deleting file");
            }
        }
    }
}

pub fn read_lines_exercise(){
    println!("\n ---Exercise 9: Read File Line by Line");

    let filename = "output_copy.txt";

    match fs::read_to_string(filename){
        Ok(contents) => {
            println!("Reading lines from {}",filename);
            for (line_num,line) in contents.lines().enumerate(){
                println!("Line : {} {}", line_num+1,line);
            }
        },
        Err(e) => println!("Error reading file : {} ",e)
    }
}

pub fn count_words_exercise(){
    println!("\n ---Exercise 10: Count words in FIle");

    let filename = "output_copy.txt";

    match fs::read_to_string(filename){
        Ok(contents) => {
            let word_count = contents.split_whitespace().count();
            let line_count = contents.lines().count();
            let char_count = contents.chars().count();

            println!("Statistics for {}",filename);
            println!("Lines: {}", line_count);
            println!("Words : {}", word_count);
            println!("Characters : {}", char_count);
        },
        Err(e)=> println!("Error while reading file")
    }
}