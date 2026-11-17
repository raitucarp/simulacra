//! Test-only HTTP/1.1 server on `127.0.0.1`: serves scripted replies in order and records
//! every request, so engine code is exercised without any real provider.

use serde_json::Value;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

const HEADER_END: &str = "\r\n";
const CONTENT_LENGTH: &str = "content-length";
const JSON_TYPE: &str = "application/json";

/// One scripted response.
#[derive(Debug, Clone)]
pub struct MockReply {
    pub status: u16,
    pub content_type: String,
    pub body: Vec<u8>,
}

impl MockReply {
    pub fn json(value: Value) -> Self {
        Self { status: 200, content_type: JSON_TYPE.into(), body: value.to_string().into_bytes() }
    }

    pub fn raw(status: u16, content_type: &str, body: impl Into<Vec<u8>>) -> Self {
        Self { status, content_type: content_type.into(), body: body.into() }
    }
}

/// A request the server received.
#[derive(Debug, Clone, Default)]
pub struct Recorded {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Recorded {
    /// The body parsed as JSON (`Null` when it is not JSON).
    pub fn json(&self) -> Value {
        serde_json::from_str(&self.body).unwrap_or(Value::Null)
    }

    /// The first header named `name` (case-insensitive).
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.iter().find(|(k, _)| k.eq_ignore_ascii_case(name)).map(|(_, v)| v.as_str())
    }
}

/// A running mock server.
pub struct MockServer {
    /// `http://127.0.0.1:<port>`.
    pub url: String,
    requests: Arc<Mutex<Vec<Recorded>>>,
}

impl MockServer {
    /// Serves `replies` to the next `replies.len()` connections, in order.
    pub fn start(replies: Vec<MockReply>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock server");
        let url = format!("http://{}", listener.local_addr().expect("local addr"));
        let requests = Arc::new(Mutex::new(Vec::new()));
        let log = requests.clone();
        std::thread::spawn(move || {
            for reply in replies {
                let Ok((stream, _)) = listener.accept() else { return };
                serve(stream, &reply, &log);
            }
        });
        Self { url, requests }
    }

    /// Every request received so far.
    pub fn requests(&self) -> Vec<Recorded> {
        self.requests.lock().expect("request log").clone()
    }
}

fn serve(stream: TcpStream, reply: &MockReply, log: &Mutex<Vec<Recorded>>) {
    let mut reader = BufReader::new(stream.try_clone().expect("clone stream"));
    let mut line = String::new();
    let _ = reader.read_line(&mut line);
    let mut parts = line.split_whitespace();
    let mut recorded = Recorded {
        method: parts.next().unwrap_or_default().to_string(),
        path: parts.next().unwrap_or_default().to_string(),
        ..Recorded::default()
    };
    loop {
        line.clear();
        if reader.read_line(&mut line).unwrap_or(0) == 0 || line == HEADER_END {
            break;
        }
        if let Some((name, value)) = line.trim_end().split_once(':') {
            recorded.headers.push((name.trim().to_string(), value.trim().to_string()));
        }
    }
    let length: usize = recorded.header(CONTENT_LENGTH).and_then(|v| v.parse().ok()).unwrap_or(0);
    let mut body = vec![0; length];
    let _ = reader.read_exact(&mut body);
    recorded.body = String::from_utf8_lossy(&body).into_owned();
    log.lock().expect("request log").push(recorded);
    let head = format!(
        "HTTP/1.1 {} MOCK\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        reply.status,
        reply.content_type,
        reply.body.len()
    );
    let mut stream = stream;
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(&reply.body);
    let _ = stream.flush();
}
