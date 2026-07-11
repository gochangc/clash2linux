use clash2linux::core::download::download_text;
use std::io::Write;
use std::net::TcpListener;
use std::thread;

fn start_test_server(body: &'static str) -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).unwrap();
            break;
        }
    });
    port
}

#[test]
fn test_download_text_from_http_server() {
    let port = start_test_server("hello mihomo");
    let url = format!("http://127.0.0.1:{}/sub", port);
    let result = download_text(&url).unwrap();
    assert_eq!(result.trim(), "hello mihomo");
}
