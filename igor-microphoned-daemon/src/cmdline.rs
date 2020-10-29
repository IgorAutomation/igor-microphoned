
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "igormd")]
pub struct Opt {

    /// List hosts
    #[structopt(long)]
    pub list_hosts: bool,

    /// Specifies the audio host to use
    #[structopt(short, long)]
    pub host: Option<String>,

    /// List devices for the specified host
    #[structopt(long)]
    pub list_devices: bool,

    /// Specifies the audio device to use
    #[structopt(short, long)]
    pub device: Option<String>,

    /// Read words from the given device
    #[structopt(long)]
    pub read_words: bool,

}
