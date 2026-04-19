# pulsemon

Cross-platform system monitor TUI with zero dependencies.

Monitors CPU, memory, disk, GPU, processes, and ports. Kill anything from the UI.

## Install

**Download a binary** from [Releases](https://github.com/aaratisharma-star/pulsemoon/releases) — pick your platform, run it.

**Or build from source:**

```
cargo install --path .
```

## Usage

```
pulsemon              # launch with defaults
pulsemon --rate 500   # 500ms refresh rate
pulsemon --no-gpu     # skip GPU detection
pulsemon --no-ports   # skip port scanning
```

## Keybindings

| Key | Action |
|-----|--------|
| `1-7` | Jump to view (Dashboard, CPU, Memory, Disk, GPU, Processes, Ports) |
| `Tab` / `Shift+Tab` | Cycle views |
| `j/k` or `↑/↓` | Navigate table rows |
| `s` / `S` | Cycle sort column / reverse sort |
| `/` | Filter by name |
| `K` or `Del` | Kill selected process/port |
| `:` | Command palette |
| `+` / `-` | Speed up / slow down refresh |
| `?` | Help |
| `q` / `Esc` | Quit or go back |

## Commands

Type `:` to open the command palette:

```
:kill 1234          Kill process by PID
:kill-port 3000     Kill process on port
:sort cpu           Sort by column (cpu, mem, pid, name, port, protocol, state)
:rate 500           Set refresh rate in ms
:filter chrome      Filter table
:q                  Quit
```

## GPU Support

| Vendor | Method | Platforms |
|--------|--------|-----------|
| NVIDIA | NVML (native) | Windows, Linux, macOS |
| AMD | rocm-smi (CLI) | Linux |
| Intel | intel_gpu_top (CLI) | Linux |
| Apple | ioreg / powermetrics | macOS |

GPU detection is best-effort — if your GPU isn't supported, pulsemon shows "No GPU detected" and everything else works fine.

## Extending

The codebase uses traits for pluggable backends:

- **GPU** — implement `GpuBackend` in `src/collectors/gpu/` and register in `detect_gpus()`
- **Ports** — implement `PortScanner` in `src/collectors/ports/` with platform-specific parsing
- **Views** — add a new view file in `src/ui/`, add a variant to `View` enum in `src/app.rs`

All colors live in `src/theme.rs` if you want to change the palette.

## Releasing

Push a tag to trigger a release build:

```
git tag v0.1.0
git push origin v0.1.0
```

GitHub Actions builds binaries for Linux, macOS (amd64 + arm64), and Windows, then creates a release.

## License

MIT
