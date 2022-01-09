use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    select,
    sync::mpsc::{self, Receiver, Sender},
};

pub struct Connections {}

impl<'a> Connections {
    pub fn add_connection(address: &str) -> (Sender<String>, Receiver<String>) {
        let (itx, mut irx) = mpsc::channel::<String>(1024);
        let (otx, orx) = mpsc::channel::<String>(1024);
        let address_string = String::from(address);
        tokio::spawn(async move {
            let mut stream = TcpStream::connect(address_string).await.unwrap();
            let (reader, mut writer) = stream.split();
            let mut buf_readerstream = BufReader::new(reader);

            loop {
                let mut buff = String::with_capacity(1024);
                select! {
                    incoming = irx.recv() => {
                        match incoming {
                            Some(value) => {
                                if let Err(_) = writer.write(value.as_bytes()).await {
                                    break;
                                }
                            }
                            None => break,
                        }
                    }
                    write =  buf_readerstream.read_line(&mut buff) => {
                        match write{
                            Ok(0) => {
                                break;
                            }
                            Err(_e) => {
                                break;
                            }
                            Ok(_) => {
                                if let Err(_) = otx.send(buff).await {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            stream.shutdown();
            irx.close();
        });
        return (itx, orx);
    }
}
