use std::fs;
use std::process;

use memoize_lib::args;
use memoize_lib::cache;
use memoize_lib::time;

fn main () {

    let (validity, cmdline) = args::parse();

    let key = cache::get_key(&cmdline);

    let cache_path = cache::get_path(&key);

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

        let timestamp = now;
        let returncode = proc.status.code().unwrap_or(-1);
        let out = proc.stdout;
        let err = proc.stderr;

        cache::overwrite(&cache_path, &out, &err, timestamp, returncode);

    }

    let output = cache::get_output(&cache_path);
    print!("{}",output);

    let error = cache::get_error(&cache_path);
    eprint!("{}",error);

    let rc = cache::get_returncode(&cache_path);
    process::exit(rc);

}
