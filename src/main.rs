use getargs::{Opt, Options};
use std::env::args;
use std::{fs, path};
use std::path::PathBuf;
use std::process::exit;

fn main() {


    const VERSION: &'static str = env!("CARGO_PKG_VERSION");




    let mut wild_card = false;

    let mut working_path = String::from("");

    let mut file_extentions:Vec<String> = Vec::new();


    let mut args = args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        args.push(String::from("--help")); // help the user out :)
    }

    let mut opts = Options::new(args.iter().map(String::as_str));

    while let Some(opt) = opts.next_opt().expect("argument parsing error") {
        match opt {
            Opt::Short('h') | Opt::Long("help") => {
                eprintln!(
                    r"Usage: filepull [OPTIONS] [ARGS]...
Take files with a certain extention and pull them into the current directory

  -h, --help       display this help and exit
  -v  --version   output version information and exit
  -f --folder      chose the folder to traverse
  -a, --all   move files of all file extentions
  -r, --removefolders   Remove Empty Folders after moving files (removes already empty folders too!)
  <anything else>  add to the list of file extentions! (only counted if they start with . (.tar.gz would work!))"
                );

                return;
            }

            
            Opt::Short('v') | Opt::Long("version") => {
                println!("filepull version {}", VERSION);
                exit(0);
            }

            Opt::Short('a') | Opt::Long("all") => {
                wild_card = true;
            }

            Opt::Short('f') | Opt::Long("folder") => {
                working_path = String::from(opts.value().expect("argument parsing error"));
            }

            _ => eprintln!("unknown option: {}", opt),
        }
    }

    for arg in opts.positionals() {
        if arg.starts_with('.') {
            let mut arg_to_use = String::from(arg);
            arg_to_use.remove(0);
            file_extentions.push(arg_to_use)
        }
    }



    if !get_current_dir().join(working_path.clone()).exists() {
        eprintln!("The path {} does not exist!", working_path);
        return;
    }

    let path_to_use = get_current_dir().join(working_path);







    let files_removed = traverse_dir(get_current_dir(),path_to_use,file_extentions, wild_card);


    print_results(files_removed);

}




//make a function that gets the current directory and puts it in a string

fn get_current_dir() -> PathBuf {
    //let current_path = std::env::current_dir().unwrap();
    // let current_path_string = current_path.to_str().unwrap();
    // return current_path_string.to_string();
    return std::env::current_dir().unwrap();
}

//make a function that take in a PathBuf and a list of file extentions, and a boolean to tell if you want to delete empty folders, checks all the folders inside and moves them into the current directory and deletes the folder if it is empty and the boolean is true.

fn traverse_dir(current_path:PathBuf, path_to_traverse: PathBuf, list:Vec<String>, wild_card: bool) -> Vec<String> {
    let mut moved_files: Vec<String> = Vec::new();


    //iterate through the files in the directory
    for entry in fs::read_dir(&path_to_traverse).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        //get the file name
        let file_name = (&path).file_name().unwrap().to_str().unwrap().to_string();

        
        if path.is_file() && (wild_card || is_valid_extention(file_name.clone(), list.clone())) {
            moved_files.push(path.to_str().unwrap().to_string());
            fs::rename(&path, (&current_path).join(&file_name)).unwrap();
        }

        if path.is_dir() {
            moved_files.append(&mut traverse_dir(current_path.clone(), path.clone(), list.clone(), wild_card));
        }

    }

    return moved_files

}

fn print_results(moved_files: Vec<String>) {
    println!("Moved Files:");
    for file in moved_files {
        println!("{}", file);
    }
}



//make a function that takes in a string of a file name and returns a boolean of if the file is has ends with an extetion in the list of file extentions
// fn is_valid_extention(file_name: String, list: Vec<String>) -> bool {

//     let file_extention = get_file_extention(file_name);
//     return list.contains(&file_extention);
// }


//make a function that checks a list for a file extention, if the extention fits inside another it also counts and returns true
fn is_valid_extention(file_name: String, list: Vec<String>) -> bool {

    let file_extention = get_file_extention(file_name);
    for extention in list.clone() {
        if extention.ends_with(&file_extention) {
            return true;
        }
    }
    return list.contains(&file_extention);
}

//make a function to get the file extention from a string of a file name
fn get_file_extention(file_name: String) -> String {
    let mut file_extention = String::new();
    let mut found_dot = false;
    for char in file_name.chars() {
        if (char == '.') && found_dot {
            file_extention = String::new();
        } else if found_dot {
            file_extention.push(char)
        }
        if char == '.' {
            found_dot = true;
        }
    }
    return file_extention;
}


// //make a function that takes in a string of a file name and returns a string of the file name with only the the extention (file.tar.gz -> .gz)
// fn get_last_file_extention(file_name: String) -> String {
//     let mut file_extention = String::new();
//     let mut found_dot = false;
//     for char in file_name.chars() {
//         if (char == '.') && found_dot {
//             file_extention = String::new();
//         } else if found_dot {
//             file_extention.push(char)
//         }
//         if char == '.' {
//             found_dot = true;
//         }
//     }
//     return file_extention;

// }