mod answer;
mod header;
mod question;

use std::net::UdpSocket;

use answer::Answer;
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
                println!("---------");
                println!("request is {:?}", &buf);

                let mut received_header = Header::default();
                if let Err(e) = received_header.parse_header(&buf[..12]) {
                    eprintln!("Failed to parse header: {}", e);
                    continue;
                }

                println!("parsed header is {:?}", received_header);

                // let mut received_question = Question::default();
                // received_question.labels = vec!["codecrafters".to_string(), "io".to_string()];
                // received_question.q_type = 1;
                // received_question.q_class = 1;

                // let mut response_header = Header::default();
                // response_header.ID = received_header.ID;
                // response_header.QR = 1;
                // response_header.QDCOUNT = 1;
                // response_header.ANCOUNT = 1;
                // response_header.RCODE = 4;
                // response_header.OPCODE = received_header.OPCODE;
                // response_header.RD = received_header.RD;

                // let mut response_answer = Answer::default();
                // response_answer.Name = vec!["codecrafters".to_string(), "io".to_string()];
                // response_answer.q_type = 1;
                // response_answer.q_class = 1;
                // response_answer.TTL = 60;
                // response_answer.Length = 4;
                // response_answer.Data = vec![8; response_answer.Length as usize];

                // let response_question = Question {
                //     labels: received_question.labels.clone(),
                //     q_type: received_question.q_type,
                //     q_class: received_question.q_class,
                // };

                // let header_bytes = response_header
                //     .create_header_as_array_of_bytes()
                //     .expect("Failed to serialize header");
                // let question_bytes = response_question
                //     .create_question_as_array_of_bytes()
                //     .expect("Failed to serialize question");
                // let answer_bytes = response_answer
                //     .create_answer_as_array_of_bytes()
                //     .expect("Failed to serialize answer");

                let mut combined = Vec::new();
                // combined.extend_from_slice(&header_bytes);
                // combined.extend_from_slice(&question_bytes);
                // combined.extend_from_slice(&answer_bytes);

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
