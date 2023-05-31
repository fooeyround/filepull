use std::fs;
use std::path::PathBuf;


pub fn get_current_dir() -> PathBuf {
    return std::env::current_dir().unwrap();
}


pub fn collect_files_to_move(current_path:PathBuf, path_to_traverse: &PathBuf, list:Vec<String>, wild_card: bool) -> Vec<PathBuf> {
    let mut files_to_move: Vec<PathBuf> = Vec::new();


    //iterate through the files in the directory
    for entry in fs::read_dir(path_to_traverse).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        //get the file name
        let file_name = (&path).file_name().unwrap().to_str().unwrap().to_string();



    
        if (&path).is_dir() {
            files_to_move.append(&mut collect_files_to_move(current_path.clone(), &path.clone(), list.clone(), wild_card));
        } else if (&path).is_file() && (wild_card || is_valid_extention(file_name.clone(), list.clone())) {
            files_to_move.push(path.clone());
        }
    
    }


    return files_to_move

}

pub fn print_file_list(moved_files: &Vec<PathBuf>) {
    for file in moved_files {
        println!("- {}", file.file_name().unwrap().to_str().unwrap());
    }
}





pub fn is_valid_extention(file_name: String, list: Vec<String>) -> bool {

    let file_extention = get_file_extention(file_name);
    for extention in list.clone() {
        if extention.ends_with(&file_extention) {
            return true;
        }
    }
    return list.contains(&file_extention);
}


pub fn get_file_extention(file_name: String) -> String {
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



pub fn move_files(file_list: &Vec<PathBuf>, to_path: &PathBuf) {
    for file in file_list {
        if file.is_file() {
            let file_name = file.file_name().unwrap().to_str().unwrap().to_string();
            fs::rename(file, (to_path).join(file_name)).unwrap();
        }
    }
}



pub fn move_file(file: &PathBuf, to_path: &PathBuf) {
    if file.is_file() {
        let file_name = file.file_name().unwrap().to_str().unwrap().to_string();
        fs::rename(file, (to_path).join(file_name)).unwrap();
    }
}




pub fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}