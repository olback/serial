use tokio_serial::SerialPortBuilderExt;

use {
    args::Args,
    clap::{CommandFactory, Parser},
    error::Result,
    std::{borrow::Cow, sync::Arc},
    tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        sync::Notify,
    },
};

mod args;
mod error;

fn enum_ports() -> Result<()> {
    let ports = tokio_serial::available_ports()?;
    println!("Available ports: {}", ports.len());
    for port in ports.iter() {
        let pt = match &port.port_type {
            tokio_serial::SerialPortType::BluetoothPort => Cow::Borrowed("Bluetooth"),
            tokio_serial::SerialPortType::PciPort => Cow::Borrowed("PCI"),
            tokio_serial::SerialPortType::UsbPort(usb) => {
                let additional = {
                    let mut additional = Vec::with_capacity(3);
                    if let Some(ref mfn) = usb.manufacturer {
                        additional.push(format!("Manufacturer: {}", mfn));
                    }
                    if let Some(ref product) = usb.product {
                        additional.push(format!("Product: {}", product));
                    }
                    if let Some(ref serial_number) = usb.serial_number {
                        additional.push(format!("Serial number: {}", serial_number));
                    }
                    additional.join(", ")
                };
                Cow::Owned(format!("{:04x}:{:04x} {}", usb.vid, usb.pid, additional))
            }
            tokio_serial::SerialPortType::Unknown => Cow::Borrowed("Unknown"),
        };
        println!("  {} ({})", port.port_name, pt);
    }

    Ok(())
}

async fn open_port(port: &str, args: &Args) -> Result<()> {
    let mut port = tokio_serial::new(port, *args.baud)
        .data_bits(args.data_bits.to_serialport_type())
        .stop_bits(args.stop_bits.to_serialport_type())
        .flow_control(args.flow_control.to_serialport_type())
        .parity(args.parity.to_serialport_type())
        .open_native_async()?;

    let mut stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();

    let mut stdin_read_buf = [0u8; 256];
    let mut serial_read_buf = [0u8; 256];

    let notify_rx = Arc::new(Notify::new());
    let notify_tx = Arc::clone(&notify_rx);
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        notify_tx.notify_waiters();
    });

    'l: loop {
        tokio::select! {
            _ = notify_rx.notified() => {
                break 'l;
            }
            serial_read_len = port.read(&mut serial_read_buf) => {
                if let Ok(len) = serial_read_len {
                    stdout.write_all(&serial_read_buf[0..len]).await?;
                    stdout.flush().await?;
                }
            }
            stdin_read_len = stdin.read(&mut stdin_read_buf) => {
                if let Ok(len) = stdin_read_len {
                    port.write_all(&stdin_read_buf[0..len]).await?;
                    port.flush().await?;
                }
            }
        }
    }

    port.shutdown().await?;
    stdout.shutdown().await?;

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();

    if let Err(e) = (async move {
        if args.enumerate {
            enum_ports()
        } else if let Some(ref port) = args.port {
            open_port(port, &args).await
        } else {
            Args::command().print_help()?;
            Ok(())
        }
    })
    .await
    {
        eprintln!("Error: {e}")
    }
}
