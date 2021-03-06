use std::error::Error;
use std::fs::File;
use std::io::{self, BufWriter, Read, Write};

///Now to write the credentials to our config file
pub fn edit_config_from_csv(
    config: &[String],
    buffer: &mut BufWriter<File>,
) -> Result<(), Box<dyn Error>> {
    let acc_key_bytes = &config[0].clone().into_bytes();
    let secr_key_bytes = &config[1].clone().into_bytes();
    buffer.get_mut().write_all(b"aws_access_key_id=")?;
    buffer.get_mut().write_all(acc_key_bytes)?;
    buffer.get_mut().write_all(b"\n")?;
    buffer.get_mut().write_all(b"aws_secret_access_key=")?;
    buffer.get_mut().write_all(secr_key_bytes)?;
    buffer.get_mut().write_all(b"\n")?;
    Ok(())
}

///Editing the credentials file using values you enter in
///standard input.(values for access key, secret key and region respectively)
pub fn edit_config_from_stdin(
    data: &str,
    buffer_file: &mut BufWriter<File>,
) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout(); //get the global stdout entity
                               //Printing to the terminal is often slow due to the calls to
                               //flush the terminal by println!()
    let handle = stdout.lock(); //acquire a lock on stdout
    let mut buffer = io::BufWriter::new(handle);
    writeln!(buffer, "{:?}", data)?; //write to stdout
    buffer.flush()?;

    {
        //This new scope is just for better readability.It's not required.

        let stdin = io::stdin();
        let mut handle = stdin.lock();
        let mut buf = String::new();
        handle.read_to_string(&mut buf)?;
        buffer_file
            .get_mut()
            .write_all(&data.to_owned().into_bytes()[..])?;
        buffer_file.get_mut().write_all(&buf.into_bytes()[..])?;
        buffer_file.get_mut().write_all(b"\n")?;
    } //end scope

    Ok(())
}
