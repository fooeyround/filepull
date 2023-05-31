use getargs::{Opt, Options};
use std::env::args;
use std::process::exit;
use indicatif::ProgressBar;

// use util::{get_current_dir, is_valid_extention, traverse_dir, print_results};
mod util;

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



    if !util::get_current_dir().join(working_path.clone()).exists() {
        eprintln!("The path {} does not exist!", working_path);
        return;
    }

    let path_to_use = util::get_current_dir().join(working_path);







    let files_to_remove = util::collect_files_to_move(util::get_current_dir(),&path_to_use,file_extentions, wild_card);



    util::clear_screen();

    println!("Are you sure you want to move these files to {}?", util::get_current_dir().to_str().unwrap());
    util::print_file_list(&files_to_remove);

    println!("(press enter to continue, anything else to abort)");


    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    if !(vec!["yes","y",""].contains(&input.trim())) {
        println!("Aborting!");
        exit(0);
    }


    util::clear_screen();



    let bar = ProgressBar::new(files_to_remove.len() as u64);

    for file in files_to_remove.clone() {
        util::move_file(&file, &path_to_use);
        bar.inc(1);
    }

    bar.finish();


    util::clear_screen();

    println!("Moved Files:");
    util::print_file_list(&files_to_remove);

}


