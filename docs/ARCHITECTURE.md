# TEZ-1200 System Architecture

## Overview

The TEZ-1200 is a Eurorack performance controller built around a brushless DC motor with a rotary encoder, driven by an ESP32-S3 microcontroller running Embassy async firmware. It translates physical knob gestures into CV, gate, and MIDI outputs with programmable haptic feedback and tempo synchronization.

## System Components

### Firmware (ESP32-S3, no_std Rust + Embassy)

The firmware runs on an ESP32-S3 (Xtensa LX7 dual-core, 240 MHz) using the `esp-hal` hardware abstraction layer and `esp-rtos` Embassy integration. All concurrency is cooperative async via Embassy's executor.

### Configurator (TypeScript)

A desktop/web tool for managing presets, uploading custom wavetables/LUTs, and configuring device parameters over USB serial.

### Hardware

Custom PCB with ESP32-S3 module, DRV8316 motor driver, AS5047P magnetic encoder, MCP4728 quad DAC, SSD1306 OLED display, and Eurorack power interface.

## Embassy Task Structure

The firmware is organized as concurrent Embassy tasks:

| Task | Priority | Rate | Description |
|------|----------|------|-------------|
| FOC control loop | High | 20 kHz | Motor commutation and torque control |
| Encoder read | High | 10 kHz | Position and velocity sampling via SPI |
| Sync clock | Medium | 1 kHz | Tempo tracking, clock division, phase sync |
| CV follower | Medium | 1 kHz | ADC sampling of external CV input |
| Output update | Medium | 1 kHz | DAC and MIDI output generation |
| UI render | Low | 30 Hz | Display updates and button scanning |
| Serial handler | Low | On-event | USB serial for configurator communication |

## Hardware Interfaces

| Peripheral | Bus | Driver IC | ESP32-S3 Pins |
|------------|-----|-----------|---------------|
| Motor driver | SPI + GPIO | DRV8316 | SPI2 + enable/fault |
| Encoder | SPI | AS5047P | SPI2 (shared) |
| DAC (CV out) | I2C | MCP4728 | I2C0 |
| OLED display | I2C | SSD1306 | I2C0 (shared) |
| Clock in/out | GPIO | - | GPIO interrupt + output |
| CV input | ADC | - | ADC1 channel |
| Gate outputs | GPIO | - | Digital outputs |
| MIDI out | UART | - | UART1 TX |
| USB serial | USB | - | Native USB-OTG |

## State Machine

The core performance state tracks:

- **SyncMode**: Internal / ExternalClock / CvFollow / Free
- **MotorMode**: Locked / Assisted / Damped / SpringReturn
- **OutputQuantization**: Continuous / BeatSynced / Chromatic / ProgrammableLut

State transitions are triggered by UI input (button combos) or serial commands from the configurator. The `PerformanceState` struct is shared across tasks via `embassy_sync::Signal` or `embassy_sync::Watch`.

## Data Flow

```
Encoder --> FOC Controller --> Motor Driver
   |              |
   v              v
Position    Haptic Feedback
   |
   v
Quantizer --> DAC Output (CV/Gate)
   |
   v
          --> MIDI Output
   |
   v
Sync Clock --> Tempo Phase
```

Preset storage uses the ESP32-S3's flash via `esp-storage`, with wear-leveling for frequent saves.
