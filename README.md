# TEZ-1200

Eurorack performance controller with haptic feedback, built around a brushless DC motor with FOC control, tempo sync, and programmable output quantization.

## Project Structure

```
firmware/       ESP32-S3 firmware (no_std Rust + Embassy)
configurator/   Configuration tool (TypeScript)
hardware/       PCB, panel, and mechanical design files
docs/           Architecture, control theory, and user documentation
```

## Getting Started

### Firmware

Requires the Espressif Rust toolchain:

```bash
cargo install espup
espup install --targets esp32s3
. $HOME/export-esp.sh

cd firmware
cargo build
```

Flash and monitor:

```bash
cargo run
```

### Configurator

```bash
cd configurator
npm install
npm run build
```

## License

- **firmware/** and **configurator/**: [GPL-3.0](LICENSE-GPL-3.0)
- **hardware/**: [CERN-OHL-S-2.0](LICENSE-CERN-OHL-S-2.0)

See [LICENSE](LICENSE) for details.
