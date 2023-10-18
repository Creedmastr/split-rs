#![feature(fs_try_exists)]

use crate::utils::join_vec;
use std::{
    fs::{self, Permissions},
    os::unix::prelude::PermissionsExt,
};
use utils::split_vec;

mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 4 {
        panic!("ERROR: Wrong number of arguments")
    };

    match &args[1] {
        x if x == "split" => {
            if args.len() < 5 {
                panic!("ERROR: Wrong number of arguments")
            };

            let f = fs::read(&args[2]).unwrap();
            let slice_number: usize = args[3].parse().unwrap();

            let splitted = split_vec(f.clone(), slice_number);

            if !fs::try_exists(&args[4]).unwrap() { 
                let _ = fs::DirBuilder::new().create(&args[4]);
            }

            for (i, u) in splitted.iter().enumerate() {
                let file_name =
                    chrono::Local::now().naive_local().to_string() + "  n" + &i.to_string();
                let _ = fs::write(format!("./{}/{}", &args[4], file_name), u);
            }
        }

        x if x == "build" => {
            let paths = fs::read_dir(&args[2]).unwrap();
            let mut splitted = vec![];
            let mut sorted_path = vec![];
            let mut current_lowest: u8 = 0;

            for path in paths {
                let path = path.unwrap();

                let last = path
                    .path()
                    .to_string_lossy()
                    .split_once(" n")
                    .unwrap()
                    .1
                    .parse::<u8>()
                    .unwrap();

                let file = fs::read(path.path()).unwrap();

                sorted_path.push(vec![file, vec![last]]);
            }

            loop {
                for path in sorted_path.clone() {
                    if current_lowest == path[1][0] {
                        splitted.push(path[0].clone());
                        current_lowest += 1;
                    }
                }

                if current_lowest > (sorted_path.len() - 1) as u8 {
                    break;
                }
            }

            let splitted = join_vec(splitted);

            let _ = fs::write(&args[3], splitted);
            let _ = fs::set_permissions(&args[3], Permissions::from_mode(0o755));
        }

        _ => panic!("ERROR: Wrong usage"),
    }
}
