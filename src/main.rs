mod answer;
mod header;
mod message;
mod question;

use message::Message;
use std::net::UdpSocket;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf: [u8; 512] = [0; 512];

    println!("DNS Server is running on 127.0.0.1:2053");

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                println!("request is {:?}", &buf);

                let mut message = Message::default();

                message.parse_message(&buf);

                let parsed_header = message.header.clone();
                let parsed_questions = message.questions.clone();
                let parsed_answer = message.answers.clone();

                println!("parsed header is {:?}", parsed_header);
                println!("parsed question is {:?}", &parsed_questions);
                println!("parsed answer is {:?}", &parsed_answer);

                let header_bytes = message
                    .header
                    .create_header_as_array_of_bytes()
                    .expect("Failed to serialize header");
                let question_bytes = message
                    .create_questions_as_array_of_bytes()
                    .expect("Failed to serialize question");
                let answer_bytes = message
                    .create_answers_as_array_of_bytes()
                    .expect("Failed to serialize answer");

                let mut combined = Vec::new();
                combined.extend_from_slice(&header_bytes);
                combined.extend_from_slice(&question_bytes);
                combined.extend_from_slice(&answer_bytes);

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
