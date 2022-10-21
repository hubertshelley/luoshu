use std::net::SocketAddr;

/// 连接trait
pub trait Connection {
    /// 发送
    fn send(&self);
    /// 接收
    fn recv(&self);
    /// 连接
    fn connected(&self);
    /// 连接关闭
    fn disconnected(&self);
    /// 获取连接host:port
    fn get_ipaddr(&self) -> SocketAddr;
}

#[cfg(test)]
mod test {
    use std::net::SocketAddr;

    use crate::connection::Connection;
    struct Con {}
    impl Connection for Con {
        fn send(&self) {}
        fn recv(&self) {}
        fn connected(&self) {}
        fn disconnected(&self) {}
        fn get_ipaddr(&self) -> SocketAddr {
            let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
            addr
        }
    }
    #[test]
    fn test_connection() {
        let con = Con {};
        con.send();
        con.recv();
        con.connected();
        con.disconnected();
        let sock_addr = con.get_ipaddr();
        assert_eq!("127.0.0.1:8080".parse(), Ok(sock_addr));
    }
}
