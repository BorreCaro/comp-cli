mod listen;
mod server;
fn main (){
    match listen::cli_listen(){
        Ok(()) => (),
        Err(e) => eprintln!("Error {e}"),
    };
}
