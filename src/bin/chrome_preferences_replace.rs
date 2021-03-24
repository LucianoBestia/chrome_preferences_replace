use chrome_preferences_replace::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // only the 1st argument (the 0 argument is the binary path+name)
    // it is the path of the preferences file
    replace_exit_type_and_exited_cleanly(&args[1]);
}
