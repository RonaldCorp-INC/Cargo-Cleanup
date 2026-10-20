use std::path::PathBuf;
use std::fs;
use std::process::Command;
use std::io;

fn get_paths_in_folder(dir: &str) -> io::Result<Vec<PathBuf>> {
    // Read the directory entries and collect them into a Vector
    let paths = fs::read_dir(dir)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();        // This whole function was voodoo for me to figure out.

    Ok(paths)
}

fn main() {
    let folders = get_paths_in_folder(".").unwrap(); // The period here signifies "current folder".
    let mut folder_count = 0;

    for path in folders {
        if path.is_dir() {
            let mut clean_command = Command::new("cargo");
            clean_command.arg("clean").current_dir(&path).status().unwrap(); // We HAVE to have .unwrap() here. Thank you, compiler.
            folder_count += 1;
        }


    }
    println!("Folders cleaned: {}", folder_count); // Added this just so that I don't FLOOD the terminal with "detected a folder..."

    // You know, with how many arguments the compiler wins, you'd think it was my wife.


}
