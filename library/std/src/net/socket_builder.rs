use crate::net::{SocketAddr, SocketAddrFamily, SocketType, TcpListener, TcpStream, UdpSocket};
use crate::sys_common::net as net_imp;
use crate::sys_common::{AsInner, FromInner, IntoInner};
use crate::{fmt, io};

/// A wrapper struct for socket.
#[unstable(feature = "socket_builder", issue = "99999")]
pub struct UnboundSocket {
    inner: net_imp::UnboundSocket,
}

impl UnboundSocket {
    /// Creates a new SockerBuilder.
    #[unstable(feature = "socket_builder", issue = "99999")]
    pub fn new(addr_family: SocketAddrFamily, sock_type: SocketType) -> io::Result<Self> {
        let inner = net_imp::UnboundSocket::new(addr_family, sock_type)?;
        Ok(Self { inner })
    }

    /// Set "reuseaddr" to true or false. This should be called before
    /// binding the socket.
    #[unstable(feature = "socket_builder", issue = "99999")]
    pub fn set_reuseaddr(self, enable: bool) -> io::Result<Self> {
        self.inner.set_reuseaddr(enable)?;
        Ok(self)
    }

    /// Binds to `addr` and returns a `UdpSocket`.
    #[unstable(feature = "socket_builder", issue = "99999")]
    pub fn bind_udp(self, addr: &SocketAddr) -> io::Result<UdpSocket> {
        let inner = self.inner.bind_udp(addr)?;
        Ok(UdpSocket::from_inner(inner))
    }

    /// Connects as a TCP client to `addr` and returns the `TcpStream`
    #[unstable(feature = "socket_builder", issue = "99999")]
    pub fn connect_tcp(self, addr: &SocketAddr) -> io::Result<TcpStream> {
        let inner = self.inner.connect_tcp(addr)?;
        Ok(TcpStream::from_inner(inner))
    }

    /// Listens at `addr` as a TCP server and returns the `TcpListener`.
    #[unstable(feature = "socket_builder", issue = "99999")]
    pub fn listen_tcp(self, addr: &SocketAddr) -> io::Result<TcpListener> {
        let inner = self.inner.listen_tcp(addr)?;
        Ok(TcpListener::from_inner(inner))
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Debug for UnboundSocket {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl AsInner<net_imp::UnboundSocket> for UnboundSocket {
    fn as_inner(&self) -> &net_imp::UnboundSocket {
        &self.inner
    }
}

impl FromInner<net_imp::UnboundSocket> for UnboundSocket {
    fn from_inner(inner: net_imp::UnboundSocket) -> UnboundSocket {
        UnboundSocket { inner }
    }
}

impl IntoInner<net_imp::UnboundSocket> for UnboundSocket {
    fn into_inner(self) -> net_imp::UnboundSocket {
        self.inner
    }
}
