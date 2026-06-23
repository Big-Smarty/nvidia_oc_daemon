# NVIDIA_OC Daemon

<p align="center">
  <a href="https://github.com/Big-Smarty/nvidia_oc_daemon">
    <img src="images/logo.png" alt="NVIDIA_OC logo" width="80" height="80">
  </a>
</p>

NVIDIA_OC Daemon is a Rust daemon for applying NVIDIA GPU overclocking settings through NVML. It is intended for systems where X.Org Coolbits-based tools such as GreenWithEnvy or `nvidia-settings` are not available or not desirable, including Wayland setups.

The daemon loads a TOML configuration from `/etc/nvidia_oc/config`, applies configured GPU clock offsets, memory clock offsets, and power limits, then watches `/etc/nvidia_oc` for changes and reloads the configuration after a debounce period.

## Warning

Overclocking and changing GPU power limits can crash the system, damage hardware, void warranties, or make a machine unbootable. This project cannot validate every GPU, driver, firmware, thermal setup, or power delivery configuration.

Use this software entirely at your own risk. Review every value before applying it, understand the limits of your hardware, and have a recovery plan before running the daemon automatically at boot.

## Features

- Applies NVIDIA GPU settings without X.Org, Coolbits, GreenWithEnvy, or `nvidia-settings`.
- Uses NVML through `nvml-wrapper` and direct NVML bindings.
- Generates `/etc/nvidia_oc/config` on first run from the currently reported GPU settings.
- Applies per-device GPU frequency offsets, memory frequency offsets, and power management limits.
- Watches `/etc/nvidia_oc` recursively and reloads modified configuration after a 5-second debounce.
- Logs to the terminal and to `demon.log` in the process working directory.

## Requirements

- Linux with a supported NVIDIA GPU.
- NVIDIA driver with NVML available as `libnvidia-ml.so`.
- Rust toolchain for building from source.
- Root privileges, or equivalent permissions, for creating `/etc/nvidia_oc/config` and applying NVML overclocking and power-limit settings.

Some laptop GPUs or driver/device combinations may not support querying or setting power limits. NVML calls can also fail if the driver, GPU, or permissions do not expose the requested operation.

## Built With

- [Rust](https://www.rust-lang.org/)
- [log](https://github.com/rust-lang/log)
- [simplelog](https://github.com/baoyachi/simple-log)
- [serde](https://github.com/serde-rs/serde)
- [toml](https://github.com/toml-rs/toml)
- [notify](https://github.com/notify-rs/notify)
- [NVIDIA Management Library](https://developer.nvidia.com/management-library-nvml)
- [nvml-wrapper](https://github.com/Cldfire/nvml-wrapper)

## Installation

Build the daemon from source:

```sh
cargo build --release
```

Install the binary somewhere appropriate for local services:

```sh
sudo install -m 0755 target/release/nvidia_oc_daemon /usr/local/bin/nvidia_oc_daemon
```

This repository does not currently ship a systemd unit. If you want the daemon to start at boot, create and manage a local service unit for your system after confirming the configuration is safe.

## Configuration

The configuration path is currently hardcoded:

```text
/etc/nvidia_oc/config
```

On first run, the daemon creates `/etc/nvidia_oc/config` by querying each detected GPU and writing the current offsets and power limit. Edit that file to the values you want the daemon to apply.

Example:

```toml
[[sets]]
device_index = 0
gpu_freq_offset = 100
mem_freq_offset = 500
power_limit = 250000

[[sets]]
device_index = 1
gpu_freq_offset = 0
mem_freq_offset = 0
power_limit = 220000
```

Fields:

- `device_index`: NVML device index to configure.
- `gpu_freq_offset`: GPU/GPC clock VF offset.
- `mem_freq_offset`: memory clock VF offset.
- `power_limit`: NVML power management limit, in milliwatts.

Run the daemon manually while testing:

```sh
sudo /usr/local/bin/nvidia_oc_daemon
```

After startup, edits under `/etc/nvidia_oc` trigger a reload. The watcher ignores rapid repeated modify events until the debounce period has elapsed.

## Caveats

- The config file path is not configurable at runtime.
- The daemon applies all configured sets on startup and after accepted config changes.
- Invalid TOML falls back to querying current GPU state rather than applying the malformed file.
- Several NVML and file operations currently fail fast if the driver, device, permissions, or filesystem state are not suitable.
- No systemd unit is included in the repository.
- The log file name is currently `demon.log`.

## Roadmap

- [x] Create daemon process.
- [x] Load configuration file.
- [x] Apply configured GPU settings.
- [ ] Add a GUI.

## License

Distributed under the MIT License. See [LICENSE.txt](LICENSE.txt) for details.

## Acknowledgments

Inspired by [Dreaming-Codes nvidia_oc](https://github.com/Dreaming-Codes/nvidia_oc/).

## Project Link

[https://github.com/Big-Smarty/nvidia_oc_daemon](https://github.com/Big-Smarty/nvidia_oc_daemon)
