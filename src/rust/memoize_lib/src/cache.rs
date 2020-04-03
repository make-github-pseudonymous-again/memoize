use std::fs;
use sha1::{Sha1, Digest};

pub fn get_key ( cmdline: &Vec<String> ) -> String {

    let mut hasher = Sha1::new();

    for arg in cmdline {
        hasher.input(arg);
        hasher.input("\0");
    }

    let hash = hasher.result();

    return format!("{:x}", hash);

}

pub fn get_default_root ( ) -> String {

    let home_dir = dirs::home_dir()
        .expect("Impossible to get your home dir!");

    return format!("{}/.cache/memoize", home_dir.display());

}

/// ```
/// use memoize_lib::cache::get_path;
/// let a = "a".to_string();
/// let b = "b".to_string();
/// let path = get_path(&a, &b);
/// assert_eq!(path, "a/b");
/// ```
pub fn get_path ( cache_root: &String, key: &String ) -> String {

    return format!("{}/{}", cache_root, key);

}

pub fn exists ( cache_path: &String ) -> bool {

    return match fs::read_dir(cache_path) {
        Ok(_) => true,
        Err(_) => false,
    } ;

}

pub fn get_timestamp ( cache_path: &String ) -> u64 {

    let cache_file = format!("{}/T", cache_path);

    let value: u64 = fs::read_to_string(cache_file)
        .expect("Could not read timestamp file.")
        .trim()
        .parse()
        .expect("Could not parse value.");

    return value;

}

pub fn get_returncode ( cache_path: &String ) -> i32 {

    let cache_file = format!("{}/R", cache_path);

    let value: i32 = fs::read_to_string(cache_file)
        .expect("Could not read return code file.")
        .trim()
        .parse()
        .expect("Could not parse value.");

    return value;

}

pub fn get_output ( cache_path: &String ) -> String {

    let cache_file = format!("{}/O", cache_path);

    return fs::read_to_string(cache_file)
        .expect("Could not read output file.");
}

pub fn get_error ( cache_path: &String ) -> String {

    let cache_file = format!("{}/E", cache_path);

    return fs::read_to_string(cache_file)
        .expect("Could not read error file.");

}

pub fn overwrite ( cache_path: &String, out: &Vec<u8>, err: &Vec<u8>, timestamp: u64, returncode: i32 ) {

    let out_cache_file = format!("{}/O", cache_path);
    let err_cache_file = format!("{}/E", cache_path);
    let tim_cache_file = format!("{}/T", cache_path);
    let ret_cache_file = format!("{}/R", cache_path);

    let tim = format!("{}", timestamp);
    let ret = format!("{}", returncode);

    fs::write(out_cache_file, out)
        .expect("Failed to write stdout file.");
    fs::write(err_cache_file, err)
        .expect("Failed to write stderr file.");
    fs::write(tim_cache_file, tim)
        .expect("Failed to write timestamp file.");
    fs::write(ret_cache_file, ret)
        .expect("Failed to write return code file.");

}

pub fn overwrite_ft ( cache_path: &String, err: &Vec<u8>, returncode: i32 ) {

    let err_cache_file = format!("{}/E", cache_path);
    let ret_cache_file = format!("{}/R", cache_path);

    let ret = format!("{}", returncode);

    fs::write(err_cache_file, err)
        .expect("Failed to write stderr file.");
    fs::write(ret_cache_file, ret)
        .expect("Failed to write return code file.");

}
