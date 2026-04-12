---
domain: Embedded Firmware Programming
category: Code & Software
verification_type: simulation
dataset_scale: ~10K+ examples from HAL libraries + RTOS samples
difficulty_range: medium/hard/superhuman
modality: code
status: strong_hypothesis
---

# Embedded Firmware Programming

## Overview
Generate firmware code (C/Rust for ARM Cortex-M, AVR, ESP32). Verification by compiling with target toolchain and running in hardware emulators (QEMU, Renode, Wokwi).

## Verification Mechanism
1. Cross-compile with arm-none-eabi-gcc or equivalent
2. Run in QEMU/Renode/Wokwi emulator
3. Check GPIO states, peripheral register values, timing constraints
4. Verify communication protocol outputs via virtual logic analyzers

## Dataset Sources
- STM32 HAL examples
- Arduino library examples
- Wokwi simulation examples
- Zephyr RTOS test suite
- FreeRTOS examples
- Embassy (Rust embedded) examples

## Task Format
**Input**: Hardware target + peripheral requirements + behavior specification
**Output**: Firmware source code (C or Rust)

## Difficulty Curriculum
1. GPIO blink, button read
2. UART/SPI/I2C communication
3. Timer interrupts, PWM generation
4. RTOS task management
5. Low-power modes, DMA transfers
6. Real-time control loops with deadline guarantees

## Connections
[[code-generation]], [[memory-management]], [[control-systems]]
