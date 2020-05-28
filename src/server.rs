use rand::{thread_rng, Rng};
use crate::game::Game;
use std::collections::HashMap;

use std::net::{TcpListener, TcpStream};
use std::thread::spawn;

use tungstenite::{accept_hdr, Message, WebSocket};
use tungstenite::handshake::server::{Request, Response};
use std::sync::{Arc, Mutex, PoisonError, MutexGuard};

const GAME_ID_CHARS: [char; 36] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5',
    '6', '7', '8', '9'];

pub struct GameSession {
    pub active: bool,
    pub game: Game,
    pub connections: Vec<WebSocket<TcpStream>>
}

fn start_session(sl: &Arc<Mutex<GameSession>>) {
    use tungstenite::Error;

    let session_lock = sl.clone();

    spawn(move || loop {
        match session_lock.lock() {
            Ok(mut session) => {
                if (*session).connections.len() == 0 {
                    (*session).active = false;
                    break;
                }

                for i in 0..(*session).connections.len() {
                    let conn = (*session).connections.get_mut(i).unwrap();

                    if conn.can_write() {
                        match conn.read_message() {
                            Ok(msg) => {
                                if msg.is_binary() || msg.is_text() {
                                    match conn.write_message(msg) {
                                        Ok(_) => {},
                                        Err(err) => match err {
                                            Error::ConnectionClosed => {
                                                println!("Connection closed");
                                                break
                                            },
                                            Error::AlreadyClosed => {
                                                error!("Connection already closed");
                                                panic!();
                                            },
                                            Error::Io(_) => {},
                                            Error::Tls(_) => {},
                                            Error::Capacity(_) => {},
                                            Error::Protocol(_) => {},
                                            Error::SendQueueFull(_) => {},
                                            Error::Utf8 => {},
                                            Error::Url(_) => {},
                                            Error::Http(_) => {},
                                            Error::HttpFormat(_) => {},
                                        },
                                    };
                                }
                            },
                            Err(err) => match err {
                                Error::ConnectionClosed => {
                                    println!("Connection closed");
                                    break
                                },
                                Error::AlreadyClosed => {
                                    error!("Error: Connection already closed");
                                    panic!();
                                },
                                Error::Io(_) => {},
                                Error::Tls(_) => {},
                                Error::Capacity(_) => {},
                                Error::Protocol(_) => {},
                                Error::SendQueueFull(_) => {},
                                Error::Utf8 => {},
                                Error::Url(_) => {},
                                Error::Http(_) => {},
                                Error::HttpFormat(_) => {},
                            }
                        }
                    } else {
                        (*session).connections.remove(i);
                        break;
                    }
                }
            },
            Err(_) => {},
        }
    });
}

fn generate_id() -> String {
    let mut rng = thread_rng();
    let mut id = String::with_capacity(4);

    for _ in 0..4 {
        id.push(GAME_ID_CHARS[rng.gen_range(0, 36)]);
    }

    return id;
}

pub struct Server {
    pub sessions: HashMap<String, Arc<Mutex<GameSession>>>
}

impl Server {
    pub fn new() -> Server {
        Server {
            sessions: HashMap::new()
        }
    }
    fn add_game(&mut self, connection: WebSocket<TcpStream>) {
        let id = loop {
            let id = generate_id();
            match self.sessions.get(&id) {
                Some(session_lock) => match session_lock.lock() {
                    Ok(session) => {
                        if !(*session).active {
                            break id;
                        }
                    },
                    Err(_) => {},
                },
                None => break id
            }
        };

        match self.sessions.get(&id) {
            Some(session_lock) => {
                match session_lock.lock() {
                    Ok(mut session) => (*session).active = true,
                    Err(_) => {}
                };
            }
            None => {
                self.sessions.insert(id.clone(), Arc::new(Mutex::new(GameSession {
                    active: true,
                    game: Game::create_game(),
                    connections: vec![]
                })));
            }
        }

        self.join_game(&id, connection);
        if let Some(session_lock) = self.sessions.get(&id) {
            start_session(session_lock);
        }
    }
    fn find_game(&mut self, id: &str) -> Option<&Arc<Mutex<GameSession>>> {
        match self.sessions.get(id) {
            Some(session_lock) => match session_lock.lock() {
                Ok(session) => {
                    if (*session).active {
                        Some(session_lock)
                    } else {
                        None
                    }
                },
                Err(_) => None
            },
            _ => None
        }
    }
    fn remove_game(&mut self, id: &String) -> bool {
        match self.sessions.remove(id) {
            Some(_) => {
                true
            },
            None => false
        }
    }
    fn join_game(&mut self, id: &str, mut websocket: WebSocket<TcpStream>) {
        match self.find_game(id) {
            Some(session_lock) => {
                match session_lock.lock() {
                    Ok(mut session) => {
                        if (*session).connections.len() < 4 {
                            if let Ok(_) = websocket.write_message(Message::Text(format!("ID: {}", id))) {
                                (*session).connections.push(websocket);
                            }
                        } else {
                            if let Ok(_) = websocket.write_message(Message::Text("Error: Game is full".into())) {
                                if let Err(e) = websocket.close(None) {
                                    error!("Error: {}", e);
                                }
                            }
                        }
                    },
                    Err(_) => {},
                }
            },
            None => {
                if let Ok(_) = websocket.write_message(Message::Text("Error: Game does not exist".into())) {
                    if let Err(e) = websocket.close(None) {
                        error!("Error: {}", e);
                    }
                }
            }
        }
    }
    fn handle_ws(&mut self, stream: TcpStream) -> Result<(), ()> {
        let mut path: String = String::new();

        let mut websocket = match accept_hdr(stream, |req: &Request, mut response: Response| {
            let mut i = 1;

            while i < req.uri().path().len() && req.uri().path().chars().nth(i).unwrap() != '/' {
                path.push(req.uri().path().chars().nth(i).unwrap());
                i += 1;
            }

            Ok(response)
        }) {
            Ok(socket) => socket,
            Err(e) => {
                error!("Error: {}", e);
                return Err(())
            },
        };

        match path.as_str() {
            "new" => {
                self.add_game(websocket);
            },
            id if id.len() == 4 &&
                        id.chars().nth(0).unwrap().is_alphabetic() &&
                        id.chars().nth(1).unwrap().is_alphabetic() &&
                        id.chars().nth(2).unwrap().is_alphabetic() &&
                        id.chars().nth(3).unwrap().is_alphabetic() => {
                self.join_game(id, websocket);
            }
            _ => {
                if let Ok(_) = websocket.write_message("Error: Invalid URI".into()) {
                    if let Err(e) = websocket.close(None) {
                        error!("Error: {}", e);
                        return Err(())
                    }
                }
            }
        }

        Ok(())
    }
    pub fn run(&mut self) -> Result<(), std::io::Error> {
        let server = TcpListener::bind("127.0.0.1:3012")?;

        for stream in server.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(e) = self.handle_ws(stream) {
                        // TODO Handle
                    }
                },
                Err(e) => {
                    error!("Error: {}", e);
                },
            }
        }

        Ok(())
    }
}