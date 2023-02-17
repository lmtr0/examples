use ssh2::*;
use ssh2::Session;
use std::{io::{Read, Write}, net};

fn main() {
    let mut sess = ssh2::Session::new().unwrap();
    sess.set_tcp_stream(net::TcpStream::connect("host:port").unwrap());
    sess.handshake().unwrap();
    sess.userauth_password("username", "password").unwrap();
    let mut channel = sess.channel_session().unwrap();
    channel.exec(command)
    let mut listener = sess.channel_direct_tcpip("/path/to/remote/socket", 8080, None).unwrap();
    // now you can do read or write as usual with the stream
    // ...
}