use codec::GameCodec;
use actix::Addr;
use actix_web::actix;
use tokio_io::io::WriteHalf;
use tokio_tcp::TcpStream;
use Server;

pub enum SessionStatus {
    Idle,
    Active
}

type NetworkWriter = actix::io::FramedWrite<WriteHalf<TcpStream>, GameCodec>;

pub struct ServerSession {
    id: usize,
    server: Addr<Server>,
    status: SessionStatus,
    writer: NetworkWriter
}

impl ServerSession {
    pub fn new(id: usize, server: Addr<Server>, writer: NetworkWriter) -> Self {
        Self {
            id,
            server,
            status: SessionStatus::Idle,
            writer
        }
    }
}