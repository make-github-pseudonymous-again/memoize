use std::fs;
use std::process;

use memoize_lib::args;
use memoize_lib::cache;
use memoize_lib::time;

fn main () {

    let (validity, cmdline) = args::parse();

    let key = cache::get_key(&cmdline);

    let cache_root = cache::get_root();
    let cache_path = cache::get_path(&cache_root, &key);

    let now = time::now();

    let fresh = match cache::exists(&cache_path) {
        true => {
            if validity < 0 {
                false
            }
            else {
                let validity = validity as u64 ;
                let previous = cache::get_timestamp(&cache_path);
                previous + validity >= now
            }
        },
        false => {
            fs::create_dir_all(&cache_path)
                .expect("Could not create cache dir");
            let out_cache_file = format!("{}/O", cache_path);
            fs::write(out_cache_file, "")
                .expect("Failed to create stdout file.");
            let tim_cache_file = format!("{}/T", cache_path);
            fs::write(tim_cache_file, "0")
                .expect("Failed to create timestamp file.");
            false
        },
    } ;

    if ! fresh {

        let executable = &cmdline[0];
        let arguments = &cmdline[1..];

        let proc = process::Command::new(executable)
            .args(arguments)
            .output()
            .expect("failed to execute process");

        let returncode = proc.status.code().unwrap_or(-1);

        let err = proc.stderr;

        if returncode == 0 {
            let timestamp = now;
            let out = proc.stdout;
            cache::overwrite(&cache_path, &out, &err, timestamp, returncode);
        }

        else {
            cache::overwrite_ft(&cache_path, &err, returncode);
        }

    }

    let output = cache::get_output(&cache_path);
    print!("{}",output);

    let error = cache::get_error(&cache_path);
    eprint!("{}",error);

    let rc = cache::get_returncode(&cache_path);
    process::exit(rc);

}
