#[macro_use] extern crate macrolisp;
use macrolisp::prelude::*;
lisp! {

    (ns
        (extern [bytes futures
                 tokio_io
                 tokio_proto
                 tokio_service])
        (use [(std {io str})
              (bytes BytesMut)
              (futures {future Future BoxFuture})
              (tokio_io {AsyncRead AsyncWrite})
              (tokio_io codec {Encoder Decoder Framed})
              (tokio_proto TcpServer)
              (tokio_proto pipeline ServerProto)
              (tokio_service Service)]))

    (defstruct LineCodec)
    (defstruct LineProto)
    (defstruct Echo)

    (defimpl (LineCodec: Decoder)
        (deftype Item String)
        (deftype Error io::Error)

        (defn decode (((self &mut Self) (buf &mut BytesMut))
                      io::Result<Option<String>>)

            (match (.position (.iter buf)
                              (lambda (b)
                                  (== (* b)
                                      b'\n')))

                ((Some(i))   (let ((line (.split_to buf i))) // remove the serialized frame from the buffer.
                            
                                 (.split_to buf 1) // Also remove the '\n'

                                 (match ((:: str::from_utf8) (& line))
                                     ((Ok(s))    (Ok (Some (.to_string s))))
                                     ((Err(_))   (Err ((:: io::Error::new) ((:: io::ErrorKind::Other) .) "invalid UTF-8"))))))

                ((None)      (Ok None)))))

    (defimpl (LineCodec: Encoder)
        (deftype Item String)
        (deftype Error io::Error)

        (defn encode (((self &mut Self) (msg String) (buf &mut BytesMut))
                      io::Result<()>)

            (.extend buf (.as_bytes msg))
            (.extend buf b"\n")
            (Ok ())))

    (defimpl <T> (LineProto: ServerProto<T>)
        (where (T AsyncRead)
               (T AsyncWrite)
               (T 'static))

        // For this protocal style, `Request` matches the `Item` type of the codec's `Encoder`
        (deftype Request String)
        // `Response` matches the `Item` type of the codec's `Decoder`
        (deftype Response String)

        // boilerplate to hook in the codec
        (deftype Transport Framed<T, LineCodec>)
        (deftype BindTransport Result<Self::Transport, io::Error>)
        (defn bind_transport (((self &Self) (io T))
                              Self::BindTransport)
            (Ok (.framed io LineCodec))))

    (defimpl (Echo: Service)
        // These types must match the corresponding protocol types;
        (deftype Request String)
        (deftype Response String)

        // For non-streaming protocols, service errors are always io::Error
        (deftype Error io::Error)

        // The future for computing the response; box it for simplicity.
        (deftype Future BoxFuture<Self::Response, Self::Error>)

        // Produce a future for computing a response from a request.
        (defn call (((self &Self) (req Self::Request))
                    Self::Future)
            // In this case, the response is immediate.
            (.boxed ((:: future::ok) req))))

    (defn main (() ())
        (let ((addr     (.unwrap (.parse "0.0.0.0:12345"))) // Specify the localhost address
              (server   ((:: TcpServer::new) LineProto addr))) // The builder requires a protocol and an address
            // We provide a way to *instantiate* the service for each new
            // connection; here, we just immediately return a new instance.
            (.serve server (lambda ()
                               (Ok Echo)))))

}
