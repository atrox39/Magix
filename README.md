# Magix 🎨

A modern image editor built with Rust and WebAssembly, featuring real-time image manipulation directly in the browser.

## Features

- 🖼️ **Image Loading**: Load and display images from your local file system
- 🎨 **Filters**:
  - Grayscale conversion
  - Color inversion
  - Brightness adjustment (-100 to +100)
- ↩️ **Undo/Redo**: Full history management for all operations
- 💾 **Export**: Save edited images as PNG
- ⚡ **High Performance**: Powered by WebAssembly for near-native speed
- 🎯 **Zero Dependencies**: Pure Rust implementation with minimal JavaScript glue

## Architecture

Built with:
- **Rust** - Core image processing logic
- **wasm-bindgen** - Rust/JavaScript interop
- **web-sys** - Web API bindings
- **Trunk** - WASM web application bundler
- **Tailwind CSS** - Modern UI styling

## Prerequisites

Before building Magix, ensure you have the following installed:

1. **Rust** (latest stable version)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **wasm32 target**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **Trunk** (WASM bundler)
   ```bash
   cargo install trunk
   ```

## Building

### Development Build

To build and run the development server with hot-reload:

```bash
trunk serve
```

This will:
- Compile the Rust code to WebAssembly
- Bundle all assets
- Start a local server at `http://localhost:8080`
- Automatically open your browser
- Watch for changes and rebuild

### Production Build

For an optimized production build:

```bash
trunk build --release
```

The output will be in the `dist/` directory, ready to be deployed to any static hosting service.

### Manual Build (without Trunk)

If you prefer to build manually:

```bash
# Build the WASM module
cargo build --target wasm32-unknown-unknown --release

# Generate JavaScript bindings
wasm-bindgen target/wasm32-unknown-unknown/release/magix.wasm --out-dir ./dist --target web
```

## Usage

### Basic Setup

```javascript
import init, { ImageEditor, EditorConfig } from "./magix.js";

// Initialize the WASM module
await init();

// Create configuration
const config = new EditorConfig("canvas");
config.file_input_id = "file-input";
config.gray_button_id = "btn-gray";
config.invert_button_id = "btn-invert";
config.brightness_slider_id = "brightness";
config.brightness_apply_id = "btn-apply-brightness";
config.undo_button_id = "btn-undo";
config.redo_button_id = "btn-redo";
config.export_button_id = "btn-export";

// Initialize the editor
const editor = new ImageEditor(config);
```

### API Reference

#### `EditorConfig`

Constructor that creates a new configuration object:

```javascript
const config = new EditorConfig(canvasId);
```

**Setters** (all optional):
- `file_input_id` - ID of file input element for loading images
- `gray_button_id` - ID of button to apply grayscale filter
- `invert_button_id` - ID of button to invert colors
- `brightness_slider_id` - ID of range input for brightness value
- `brightness_apply_id` - ID of button to apply brightness adjustment
- `undo_button_id` - ID of button for undo operation
- `redo_button_id` - ID of button for redo operation
- `export_button_id` - ID of button to export image

#### `ImageEditor`

Main editor class that handles all image operations:

```javascript
const editor = new ImageEditor(config);
```

All event handlers are automatically attached based on the provided configuration.

## Project Structure

```
magix/
├── src/
│   ├── lib.rs              # Library entry point
│   ├── api.rs              # WASM API and public interface
│   ├── editor/
│   │   ├── mod.rs          # Editor module
│   │   ├── state.rs        # Editor state management
│   │   ├── filters.rs      # Image filter implementations
│   │   ├── history.rs      # Undo/redo history
│   │   └── loader.rs       # Image loading utilities
│   └── dom/
│       ├── mod.rs          # DOM module
│       ├── attach.rs       # Event handler attachments
│       └── utils.rs        # DOM utility functions
├── Cargo.toml              # Rust dependencies
├── Trunk.toml              # Trunk configuration
├── index.html              # Main HTML file
└── README.md               # This file
```

## Performance Optimization

The release build is optimized for size and performance:

```toml
[profile.release]
opt-level = "z"      # Optimize for size
lto = true           # Link-time optimization
codegen-units = 1    # Better optimization, slower compile
panic = "abort"      # Smaller binary size
```

## Browser Compatibility

Magix works in all modern browsers that support WebAssembly:
- Chrome/Edge 57+
- Firefox 52+
- Safari 11+
- Opera 44+

## Contributing

Contributions are welcome! Feel free to submit issues and pull requests.

## License

This project is open source and available under the MIT License.

## Author

Created by **atrox39**

---

**Enjoy editing images with the power of Rust! 🦀**
