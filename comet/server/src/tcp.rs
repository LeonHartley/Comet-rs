use actix::Addr;

use Server;

struct TcpServer {
    server: Addr<Server>
}

impl Actor for TcpServer {
    type Context = Context<Self>;
}

#[derive(Message)]
struct TcpConnect(TcpStream);

/// Handle stream of TcpStream's
impl Handler<TcpConnect> for TcpServer {
    type Result = ();

    fn handle(&mut self, msg: TcpConnect, _: &mut Context<Self>) {

    }
}
