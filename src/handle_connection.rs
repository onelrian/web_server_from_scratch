use std::{
    fs,
    io::{prelude::*, BufReader},
    net::TcpStream,
    thread,
    time::Duration,
};

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    {
        let (status_line, filename) = match &request_line[..] {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
            "GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "hello.html")
            }
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        };

        let content = fs::read_to_string(filename).unwrap();
        let length = content.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

        stream.write_all(response.as_bytes()).unwrap()
    }
}

/////////////incase you want to view the request content/////////////////
// let buf_reader = BufReader::new(&stream);                           //
// let request_line = buf_reader.lines().next().unwrap().unwrap();     //
//                                                                     //
// let http_request: Vec<_> = buf_reader                               //
//     .lines()                                                        //
//     .map(|result| result.unwrap())                                  //
//     .take_while(|line| !line.is_empty())                            //
//     .collect();                                                     //
/////////////////////////////////////////////////////////////////////////
