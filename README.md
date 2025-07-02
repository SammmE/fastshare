# FastShare 🚀

A lightning-fast command-line file sharing tool for transferring files between devices on the same network.

## Features

- ⚡ **Ultra-fast transfers** with optimized TCP streaming
- 📊 **Real-time progress bar** with transfer speed and ETA
- 🌐 **Cross-platform** - works on Windows, macOS, and Linux
- 🔧 **Simple CLI** - just two commands to send and receive
- 🛡️ **Local network only** - secure transfers within your WiFi network

## Installation

1. Make sure you have [Rust](https://rustup.rs/) installed
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/fastshare
   cd fastshare
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```
4. The binary will be available at `target/release/fastshare`

## Usage

### Sending a file

On the device that has the file you want to share:

```bash
fastshare send path/to/your/file.txt
```

This will:
- Display your local IP address
- Start listening for connections
- Show a command for the receiving device

Example output:
```
🚀 FastShare Sender
📁 File: document.pdf (2,456,789 bytes)
🌐 Listening on: 192.168.1.100:8080
📱 On the receiving device, run:
   fastshare receive 192.168.1.100

⏳ Waiting for connection...
```

### Receiving a file

On the device where you want to receive the file:

```bash
fastshare receive 192.168.1.100
```

This will connect to the sender and download the file to the current directory.

Example output:
```
🚀 FastShare Receiver
🔗 Connecting to 192.168.1.100:8080...
✅ Connected!
📁 Receiving: document.pdf (2,456,789 bytes)
📥 Starting file transfer...
⠋ [00:00:02] [##########>           ] 1.2MB/2.4MB (600KB/s, 00:00:02)
```

## Command Options

### Send Command
```bash
fastshare send <FILE> [OPTIONS]
```

Options:
- `-p, --port <PORT>` - Custom port (default: 8080)

### Receive Command
```bash
fastshare receive <IP> [OPTIONS]
```

Options:
- `-p, --port <PORT>` - Custom port (default: 8080)
- `-o, --output <DIR>` - Output directory (default: current directory)

## Examples

### Send a file on a custom port
```bash
fastshare send my-file.zip --port 9000
```

### Receive to a specific directory
```bash
fastshare receive 192.168.1.100 --output ~/Downloads
```

### Transfer between different platforms
```bash
# On Windows (sender)
fastshare send C:\Users\John\Documents\presentation.pptx

# On macOS (receiver)
fastshare receive 192.168.1.100 --output ~/Desktop
```

## Performance

FastShare is optimized for speed:
- Uses efficient 64KB chunks for optimal throughput
- Minimal protocol overhead
- Direct TCP streaming without compression (for maximum speed)
- Progress tracking with minimal performance impact

Typical transfer speeds on a modern WiFi network:
- **Local WiFi (5GHz)**: 50-100 MB/s
- **Local WiFi (2.4GHz)**: 10-25 MB/s
- **Ethernet**: 100+ MB/s

## Security

FastShare is designed for trusted local networks:
- Only works on the same network (no internet routing)
- No authentication (assume trusted network)
- No encryption (prioritizes speed over security)
- Files are transferred directly without cloud storage

⚠️ **Warning**: Only use FastShare on trusted networks. Do not use on public WiFi or untrusted networks.

## Troubleshooting

### Connection Issues
- Ensure both devices are on the same WiFi network
- Check if firewall is blocking the port (default: 8080)
- Try a different port using `--port` option

### Permission Issues
- Make sure you have read permissions for the source file
- Ensure write permissions for the destination directory

### Large Files
- FastShare can handle files of any size
- For very large files (>1GB), ensure stable network connection
- Monitor available disk space on receiving device

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) for maximum performance
- Uses [Tokio](https://tokio.rs/) for async networking
- Progress bars powered by [indicatif](https://github.com/console-rs/indicatif)
- CLI interface with [clap](https://github.com/clap-rs/clap)
