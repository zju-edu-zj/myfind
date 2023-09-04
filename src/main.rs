use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::process;


fn find<P:AsRef<Path>>(root:P,regex:&Regex,verbose:bool)->Result<Vec<String>,Box<dyn std::error::Error>>{
    let mut matches = Vec::new();
    walk_tree(root.as_ref(), regex, &mut matches,verbose)?; //return if error
    Ok(matches)
}

fn walk_tree(
    dir:&Path,
    regex:&Regex,
    matches:&mut Vec<String>,
    verbose:bool
) -> Result<(),Box<dyn std::error::Error>>{
    if dir.is_dir(){ //must be a directory if the input is right
        for entry in fs::read_dir(dir)?{
            let entry = entry?;
            let path = entry.path();
            if path.is_dir(){
                walk_tree(&path, regex, matches,verbose)?;//return directly when error occurs
            }else if let Some(filename) = path.file_name().and_then(|s| s.to_str()){
                if regex.is_match(filename){
                    matches.push(path.to_string_lossy().to_string());
                }else if verbose{ //push anyway if verbose
                    matches.push(path.to_string_lossy().to_string());
                }
            }   
        }
    }
    //else{
    //     eprintln!("the input isn't a directory");
    //     Err("the")
    // }
    Ok(())
}

fn main() {
    //println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    if args.len()<3{
        eprintln!("Correct method: {}<target directory><regex taken>",args[0]);
        process::exit(1);
    }
    let mut verbose = false;
    let mut pattern = &args[2];
    let mut root = &args[1];
    let ver = &args[1];
    if ver.eq("-v"){
        verbose = true;
        pattern = &args[3]; //move to next argument
        root = &args[2];
    }
    let regex = match Regex::new(pattern){
        Ok(re) => re,
        Err(err) =>{
            eprintln!("unvalid '{}':'{}'",pattern,err);
            process::exit(1);
        }
    };

    match find(root,&regex,verbose){
        Ok(matches)=>{
            if matches.is_empty(){
                println!("No find.");
            }else{
                println!("The results are as as follows:");
                for file in matches{
                    println!("{}",file);
                }
            }
        }
        Err(error)=>{
            eprintln!("Error:{}",error);
            process::exit(1);
        }
    }
}
