use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const CHUNK_SIZE: usize = 64 * 1024; // 64KB chunks
const PORT: u16 = 8080;

fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

#[derive(Parser)]
#[command(name = "fastshare")]
#[command(about = "Ultra-fast file sharing between devices on the same network")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send a file to another device
    Send {
        /// Path to the file to send
        #[arg(value_name = "FILE")]
        file: PathBuf,
        /// Port to listen on (default: 8080)
        #[arg(short, long, default_value_t = PORT)]
        port: u16,
    },
    /// Receive a file from another device
    Receive {
        /// IP address of the sender
        #[arg(value_name = "IP")]
        ip: String,
        /// Port to connect to (default: 8080)
        #[arg(short, long, default_value_t = PORT)]
        port: u16,
        /// Output directory (default: current directory)
        #[arg(short, long, default_value = ".")]
        output: PathBuf,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct FileInfo {
    name: String,
    size: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Send { file, port } => send_file(file, port).await,
        Commands::Receive { ip, port, output } => receive_file(ip, port, output).await,
    }
}

async fn send_file(file_path: PathBuf, port: u16) -> Result<()> {
    // Get file info
    let file_size = tokio::fs::metadata(&file_path)
        .await
        .context("Failed to get file metadata")?
        .len();
    
    let file_name = file_path
        .file_name()
        .context("Invalid file path")?
        .to_string_lossy()
        .to_string();

    // Get local IP
    let local_ip = local_ip().context("Failed to get local IP address")?;
    
    println!("🚀 FastShare Sender");
    println!("📁 File: {} ({})", file_name, format_file_size(file_size));
    println!("🌐 Listening on: {}:{}", local_ip, port);
    println!("📱 On the receiving device, run:");
    println!("   fastshare receive {}", local_ip);
    println!();

    // Start TCP listener
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .context("Failed to bind to port")?;

    println!("⏳ Waiting for connection...");

    // Accept connection
    let (mut stream, addr) = listener.accept().await.context("Failed to accept connection")?;
    println!("✅ Connected to: {}", addr);

    // Send file info
    let file_info = FileInfo {
        name: file_name.clone(),
        size: file_size,
    };

    let encoded = bincode::serialize(&file_info).context("Failed to serialize file info")?;
    stream.write_u32(encoded.len() as u32).await?;
    stream.write_all(&encoded).await?;

    println!("📤 Starting file transfer...");

    // Create progress bar
    let pb = ProgressBar::new(file_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    // Read and send file in chunks
    let mut file = File::open(&file_path).await.context("Failed to open file")?;
    let mut buffer = vec![0u8; CHUNK_SIZE];
    let mut total_sent = 0u64;

    loop {
        let bytes_read = file.read(&mut buffer).await.context("Failed to read file")?;
        if bytes_read == 0 {
            break;
        }

        stream.write_all(&buffer[..bytes_read]).await.context("Failed to send data")?;
        total_sent += bytes_read as u64;
        pb.set_position(total_sent);
    }

    pb.finish_with_message("✅ Transfer complete!");
    println!("🎉 File sent successfully!");

    Ok(())
}

async fn receive_file(ip: String, port: u16, output_dir: PathBuf) -> Result<()> {
    println!("🚀 FastShare Receiver");
    println!("🔗 Connecting to {}:{}...", ip, port);

    // Connect to sender
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port))
        .await
        .context("Failed to connect to sender")?;

    println!("✅ Connected!");

    // Receive file info
    let len = stream.read_u32().await?;
    let mut buf = vec![0u8; len as usize];
    stream.read_exact(&mut buf).await?;
    
    let file_info: FileInfo = bincode::deserialize(&buf)
        .context("Failed to deserialize file info")?;

    println!("📁 Receiving: {} ({})", file_info.name, format_file_size(file_info.size));

    // Create output file
    let output_path = output_dir.join(&file_info.name);
    let mut output_file = File::create(&output_path)
        .await
        .context("Failed to create output file")?;

    // Create progress bar
    let pb = ProgressBar::new(file_info.size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    println!("📥 Starting file transfer...");

    // Receive file data
    let mut buffer = vec![0u8; CHUNK_SIZE];
    let mut total_received = 0u64;

    while total_received < file_info.size {
        let bytes_to_read = CHUNK_SIZE.min((file_info.size - total_received) as usize);
        buffer.resize(bytes_to_read, 0);
        
        stream.read_exact(&mut buffer).await.context("Failed to receive data")?;
        output_file.write_all(&buffer).await.context("Failed to write to file")?;
        
        total_received += bytes_to_read as u64;
        pb.set_position(total_received);
    }

    output_file.flush().await?;
    pb.finish_with_message("✅ Transfer complete!");
    
    println!("🎉 File received successfully!");
    println!("📁 Saved to: {}", output_path.display());

    Ok(())
}