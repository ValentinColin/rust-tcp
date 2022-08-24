use std::net::TcpStream;
use std::io::{Write, Read, stdin};

fn get_entry() -> String {
    let mut buf = String::new();

    stdin().read_line(&mut buf).expect("");
    buf.replace("\n", "").replace("\r", "")
}

fn exchange_with_server(mut stream: TcpStream) {
    let stdout = std::io::stdout();
    let mut io = stdout.lock();
    let buf = &mut [0; 3];

    println!("Enter 'quit' when you want to leave");
    loop {
        write!(io, "> ").expect("");
        // pour afficher de suite
        io.flush().expect("");
        match &*get_entry() {
            "quit" => {
                println!("bye !");
                return;
            }
            line => {
                write!(stream, "{}\n", line).expect("");
                match stream.read(buf) {
                    Ok(received) => {
                        if received < 1 {
                            println!("Perte de la connexion avec le serveur");
                            return;
                        }
                    }
                    Err(_) => {
                        println!("Perte de la connexion avec le serveur");
                        return;
                    }
                }
                println!("Réponse du serveur : {}", String::from_utf8_lossy(buf));
            }
        }
    }
}

fn main() {
    println!("Tentative de connexion au serveur...");
    match TcpStream::connect("127.0.0.1:1234") {
        Ok(stream) => {
            println!("Connexion au serveur réussie !");
            exchange_with_server(stream);
        }
        Err(e) => {
            println!("La connexion au serveur a échoué : {}", e);
        }
    }
}
