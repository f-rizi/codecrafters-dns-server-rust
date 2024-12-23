mod header;
mod question;

use std::net::UdpSocket;

use header::Header;
use question::Question;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf: [u8; 512] = [0; 512];

    println!("DNS Server is running on 127.0.0.1:2053");

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let mut received_header = Header::default();
                if let Err(e) = received_header.parse_header(&buf[..size]) {
                    eprintln!("Failed to parse header: {}", e);
                    continue;
                }


                let mut received_question = Question::default();
                received_question.labels = vec!["codecrafters".to_string(), "io".to_string()];
                received_question.q_type = 1;
                received_question.q_class = 1;

                let mut response_header = Header::default();
                response_header.ID = received_header.ID;
                response_header.QR = 1; 
                response_header.QDCOUNT = received_header.QDCOUNT;
                response_header.ANCOUNT = 1; 
                response_header.RCODE = 0; 

                let response_question = Question {
                    labels: received_question.labels.clone(),
                    q_type: received_question.q_type,
                    q_class: received_question.q_class,
                };

                let header_bytes = response_header
                    .create_header_as_array_of_bytes()
                    .expect("Failed to serialize header");
                let question_bytes = response_question
                    .create_question_as_array_of_bytes()
                    .expect("Failed to serialize question");

                let mut combined = Vec::new();
                combined.extend_from_slice(&header_bytes);
                combined.extend_from_slice(&question_bytes);

                udp_socket
                    .send_to(&combined, source)
                    .expect("Failed to send response");

                println!("Sent response to {}", source);
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
