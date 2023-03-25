fn main() {
    if let Err(err) = commr::get_args().and_then(commr::run) {

        eprintln!("{}",err);
        std::process::exit(1);
    }
}
