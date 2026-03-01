# GTK Layer Shell Test

A Tauri-based application for testing GTK layer shell functionality with system monitoring capabilities.

## Features

- **System Monitoring**: Real-time monitoring of CPU, RAM, Disk, and Network usage
- **Media Controls**: Integration with media players for playback control
- **GTK Layer Shell**: Native GTK integration with layer shell support
- **Modern UI**: Vue.js frontend with responsive design

## Requirements

- Rust 1.70+
- GTK 4.0+
- Tauri CLI
- Node.js 16+ and npm/pnpm

## Installation

1. Install Rust and Cargo

2. Install Tauri CLI
3. Clone and install dependencies:
   ```bash
   git clone https://github.com/nitkanikita21/gtk-layer-shell-test.git
   cd gtk-layer-shell-test
   pnpm install
   ```

4. Build and run:
   ```bash
   pnpm tauri dev
   ```

## Building for Production

```bash
pnpm tauri build
```

## Development

The project uses:
- **Frontend**: Vue 3 with TypeScript
- **Backend**: Rust with Tauri
- **Styling**: CSS + TailwindCSS
- **Package Manager**: pnpm

## Architecture

- `src/` - Vue.js frontend components
- `src-tauri/` - Rust backend with Tauri modules
- `src-tauri/src/modules/` - Custom Tauri modules for shell

## License

MIT License