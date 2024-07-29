use ftp::FtpStream;
use std::str;
use std::env;

pub fn getmessage(machine_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    
    let ftp_conn = env::var("FTP_CONN").unwrap_or_else(|_| "localhost:21".into());
    let mut ftp_stream = FtpStream::connect(ftp_conn).unwrap();
    let _ = ftp_stream.login("one", "1234ftpuser").unwrap();

    println!("FTP connection established");
    println!("Current directory: {}", ftp_stream.pwd().unwrap());
    ftp_stream.cwd("/data").unwrap();
    let remote_file = ftp_stream.simple_retr("samplenews.xml").unwrap();
    let binding = remote_file.into_inner();
    let input = str::from_utf8(&binding).unwrap();
    let _ = ftp_stream.quit();

    Ok(input.into())   
}
