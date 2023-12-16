use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
mod log;
mod threadpool;

fn main() {
    let tag_client = "CLIENT";
    let tag_error = "ERROR";
    // TODO: Instead of panic, catch the error and prevent only logging
    // init logging
    let log_file = &mut log::init_log("server_log_file.txt").expect("Log file error");
    // Bind server to port and listen
    let listener = TcpListener::bind("127.0.0.1:7878");
    let pool = threadpool::ThreadPool::new(4);
    match listener{
        Ok(server) => {
            for stream in server.incoming(){
                match stream{
                    Ok(stream) => {
                        match stream.local_addr(){
                            Ok(socket) => {
                                let client = socket.ip();
                                let msg = format!("Connection Established {client}");
                                log::print_log(log_file, tag_client, &msg);
                                pool.execute(|| {
                                    handle_client_request(stream);
                                });
                            }
                            Err(err) => {
                                log::print_log(log_file, tag_error, &err.to_string());
                            }
                        }
                    }
                    Err(e) => {
                        log::print_log(log_file, tag_error, &e.to_string());
                    }
                }
            }
        }
        Err(err)=> {
            log::print_log(log_file, tag_error, &err.to_string());
        }
    }
}

fn handle_client_request(mut stream: TcpStream){
    thread::sleep(Duration::from_secs(3)); // lets add some work
    let buf_reader = BufReader::new(&mut stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap()) // TODO: Handle This
        .take_while(|line| !line.is_empty())
        .collect();
    // TODO: Log clients requests here, how should I log this concurrently!
    // println!("Request: {:#?}", http_request);
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap(); // TODO: Handle this

}




















