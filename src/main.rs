mod answer;
mod header;
mod message;
mod question;

use answer::Answer;
use futures::future::join_all;
use message::Message;
use question::Question;

use rand::random;
use std::env;
use std::error::Error;
use std::sync::Arc;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut redirect_address = String::new();

    if args.len() >= 3 {
        let key = args[1].clone();
        let value = args[2].clone();

        if key == "--resolver" {
            redirect_address = value;
        }
    }

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").await?;
    let udp_socket = Arc::new(udp_socket);
    let redirect_address = Arc::new(redirect_address);

    loop {
        let udp_socket = Arc::clone(&udp_socket);
        let redirect_address = Arc::clone(&redirect_address);

        let mut buf = vec![0u8; 512];

        // Receive data from client
        let (size, source) = match udp_socket.recv_from(&mut buf).await {
            Ok((size, src)) => (size, src),
            Err(e) => {
                eprintln!("Failed to receive data: {}", e);
                continue;
            }
        };

        let packet = buf[..size].to_vec();

        // Spawn a task to handle the DNS request
        tokio::spawn(async move {
            if let Err(e) = handle_client(&udp_socket, &redirect_address, packet, source).await {
                eprintln!("Error handling client {}: {}", source, e);
            }
        });
    }
}

async fn handle_client(
    udp_socket: &Arc<UdpSocket>,
    redirect_address: &Arc<String>,
    packet: Vec<u8>,
    source: std::net::SocketAddr,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut client_message = Message::default();
    if let Err(e) = client_message.parse_message(&packet) {
        eprintln!("Failed to parse DNS request from {}: {}", source, e);
        return Ok(());
    }

    let questions = client_message.questions.clone();

    if questions.is_empty() {
        eprintln!("No questions found in the DNS request from {}", source);
        return Ok(());
    }

    if redirect_address.is_empty() {
        client_message.header.QR = 1;
        client_message.header.ANCOUNT = client_message.header.QDCOUNT;
        client_message.answers = client_message.parse_answers()?;

        let response_bytes = client_message.create_response_bytes()?;

        udp_socket.send_to(&response_bytes, source).await?;
    } else {
        let resolver_addr = redirect_address.as_str().to_string();

        let resolver_socket = UdpSocket::bind("0.0.0.0:0").await?;
        let resolver_socket = Arc::new(resolver_socket);

        let resolution_futures = questions.iter().map(|question| {
            let resolver_socket = Arc::clone(&resolver_socket);
            let resolver_addr = resolver_addr.clone();
            let question_clone = question.clone();
            async move { resolve_question(&question_clone, &resolver_socket, &resolver_addr).await }
        });

        let results = join_all(resolution_futures).await;

        let mut combined_answers = Vec::new();
        for result in results {
            match result {
                Ok(answer) => {
                    combined_answers.push(answer);
                }
                Err(e) => {}
            }
        }

        client_message.header.QR = 1;
        client_message.header.ANCOUNT = combined_answers.len() as u16;
        client_message.answers = combined_answers;

        let response_bytes = client_message.create_response_bytes()?;

        udp_socket.send_to(&response_bytes, source).await?;
    }

    Ok(())
}

async fn resolve_question(
    question: &Question,
    resolver_socket: &Arc<UdpSocket>,
    resolver_addr: &str,
) -> Result<Answer, Box<dyn Error + Send + Sync>> {
    let mut query_message = Message::default();
    query_message.header.ID = random::<u16>();
    query_message.header.QR = 0;
    query_message.header.OPCODE = 0;
    query_message.header.AA = 0;
    query_message.header.TC = 0;
    query_message.header.RD = 1;
    query_message.header.RA = 0;
    query_message.header.Z = 0;
    query_message.header.RCODE = 0;
    query_message.header.QDCOUNT = 1;
    query_message.questions.push(question.clone());

    let query_bytes = query_message.create_response_bytes()?;

    resolver_socket.send_to(&query_bytes, resolver_addr).await?;

    let mut answer_buf = [0u8; 512];

    let recv_result = resolver_socket.recv_from(&mut answer_buf).await?;

    let mut response_message = Message::default();
    response_message.parse_message(&answer_buf[..recv_result.0])?;

    if let Some(answer) = response_message.answers.first() {
        Ok(answer.clone())
    } else {
        Err("No answer received".into())
    }
}
