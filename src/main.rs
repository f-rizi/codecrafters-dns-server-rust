mod header;
mod question;

use std::f64::consts::E;
#[allow(unused_imports)]
use std::net::UdpSocket;

use header::Header;
use question::Question;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf: [u8; 512] = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let mut header = Header::default();
                header.ID = 1234;
                header.QR = 1;
                header.QDCOUNT = 1;

                let rest = buf[12..].to_vec();
                let mut question = Question::default();
                question.labels = vec!["codecrafters".to_string(), "io".to_string()];
                question.q_type = 1;
                question.q_class = 1;

                let header_result: Result<[u8; 12], &str> =
                    header.create_header_as_array_of_bytes();
                let question_result: Result<Vec<u8>, &str> =
                    question.create_quetion_as_array_of_bytes();

                match header_result {
                    Ok(header) => {
                        match question_result {
                            Ok(question) => {
                                let mut combined = Vec::new();
                                combined.extend_from_slice(&header); 
                                combined.extend_from_slice(&question);

                                udp_socket
                                    .send_to(&combined, source)
                                    .expect("Failed to send response");
                            }
                            Err(_) => {}
                        }
                    }
                    Err(_) => {}
                }
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
