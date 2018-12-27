(function() {var implementors = {};
implementors["hyper"] = [{text:"impl <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"struct\" href=\"hyper/upgrade/struct.Upgraded.html\" title=\"struct hyper::upgrade::Upgraded\">Upgraded</a>",synthetic:false,types:["hyper::upgrade::Upgraded"]},];
implementors["hyper_rustls"] = [{text:"impl&lt;T:&nbsp;<a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"tokio_io/async_write/trait.AsyncWrite.html\" title=\"trait tokio_io::async_write::AsyncWrite\">AsyncWrite</a>&gt; <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"enum\" href=\"hyper_rustls/enum.MaybeHttpsStream.html\" title=\"enum hyper_rustls::MaybeHttpsStream\">MaybeHttpsStream</a>&lt;T&gt;",synthetic:false,types:["hyper_rustls::stream::MaybeHttpsStream"]},];
implementors["tokio_core"] = [{text:"impl <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"struct\" href=\"tokio_core/net/struct.TcpStream.html\" title=\"struct tokio_core::net::TcpStream\">TcpStream</a>",synthetic:false,types:["tokio_core::net::tcp::TcpStream"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for &amp;'a <a class=\"struct\" href=\"tokio_core/net/struct.TcpStream.html\" title=\"struct tokio_core::net::TcpStream\">TcpStream</a>",synthetic:false,types:["tokio_core::net::tcp::TcpStream"]},{text:"impl&lt;E:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt; <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"struct\" href=\"tokio_core/reactor/struct.PollEvented.html\" title=\"struct tokio_core::reactor::PollEvented\">PollEvented</a>&lt;E&gt;",synthetic:false,types:["tokio_core::reactor::poll_evented::PollEvented"]},{text:"impl&lt;'a, E&gt; <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for &amp;'a <a class=\"struct\" href=\"tokio_core/reactor/struct.PollEvented.html\" title=\"struct tokio_core::reactor::PollEvented\">PollEvented</a>&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'a </a>E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>,&nbsp;</span>",synthetic:false,types:["tokio_core::reactor::poll_evented::PollEvented"]},];
implementors["tokio_fs"] = [{text:"impl <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"struct\" href=\"tokio_fs/file/struct.File.html\" title=\"struct tokio_fs::file::File\">File</a>",synthetic:false,types:["tokio_fs::file::File"]},{text:"impl <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"struct\" href=\"tokio_fs/struct.Stdin.html\" title=\"struct tokio_fs::Stdin\">Stdin</a>",synthetic:false,types:["tokio_fs::stdin::Stdin"]},];
implementors["tokio_reactor"] = [{text:"impl&lt;E&gt; <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"struct\" href=\"tokio_reactor/struct.PollEvented.html\" title=\"struct tokio_reactor::PollEvented\">PollEvented</a>&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"mio/event_imp/trait.Evented.html\" title=\"trait mio::event_imp::Evented\">Evented</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>,&nbsp;</span>",synthetic:false,types:["tokio_reactor::poll_evented::PollEvented"]},{text:"impl&lt;'a, E&gt; <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for &amp;'a <a class=\"struct\" href=\"tokio_reactor/struct.PollEvented.html\" title=\"struct tokio_reactor::PollEvented\">PollEvented</a>&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"mio/event_imp/trait.Evented.html\" title=\"trait mio::event_imp::Evented\">Evented</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'a </a>E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>,&nbsp;</span>",synthetic:false,types:["tokio_reactor::poll_evented::PollEvented"]},];
implementors["tokio_rustls"] = [{text:"impl&lt;S, C&gt; <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"struct\" href=\"tokio_rustls/struct.TlsStream.html\" title=\"struct tokio_rustls::TlsStream\">TlsStream</a>&lt;S, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"tokio_io/async_write/trait.AsyncWrite.html\" title=\"trait tokio_io::async_write::AsyncWrite\">AsyncWrite</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"rustls/session/trait.Session.html\" title=\"trait rustls::session::Session\">Session</a>,&nbsp;</span>",synthetic:false,types:["tokio_rustls::TlsStream"]},];
implementors["tokio_tcp"] = [{text:"impl <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"struct\" href=\"tokio_tcp/struct.TcpStream.html\" title=\"struct tokio_tcp::TcpStream\">TcpStream</a>",synthetic:false,types:["tokio_tcp::stream::TcpStream"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for &amp;'a <a class=\"struct\" href=\"tokio_tcp/struct.TcpStream.html\" title=\"struct tokio_tcp::TcpStream\">TcpStream</a>",synthetic:false,types:["tokio_tcp::stream::TcpStream"]},];
implementors["tokio_uds"] = [{text:"impl <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for <a class=\"struct\" href=\"tokio_uds/struct.UnixStream.html\" title=\"struct tokio_uds::UnixStream\">UnixStream</a>",synthetic:false,types:["tokio_uds::stream::UnixStream"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> for &amp;'a <a class=\"struct\" href=\"tokio_uds/struct.UnixStream.html\" title=\"struct tokio_uds::UnixStream\">UnixStream</a>",synthetic:false,types:["tokio_uds::stream::UnixStream"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
