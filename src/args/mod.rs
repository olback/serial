use {
    baud::Baud, clap::Parser, data_bits::DataBits, flow_control::FlowControl, parity::Parity,
    stop_bits::StopBits,
};

mod baud;
mod data_bits;
mod flow_control;
mod parity;
mod stop_bits;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub port: Option<String>,

    #[arg(short, long, default_value_t = Baud::new(115200))]
    pub baud: Baud,

    #[arg(long, value_enum, default_value_t = DataBits::Eight)]
    pub data_bits: DataBits,

    #[arg(long, value_enum, default_value_t = StopBits::One)]
    pub stop_bits: StopBits,

    #[arg(long, value_enum, default_value_t = Parity::None)]
    pub parity: Parity,

    #[arg(long, value_enum, default_value_t = FlowControl::None)]
    pub flow_control: FlowControl,

    #[arg(short, long, default_value_t = false)]
    pub enumerate: bool,
}
