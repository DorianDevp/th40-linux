# th40-via9

`th40-via9` is a small Linux CLI for remapping an EPOMAKER TH40 that is visible to VIA but cannot be configured by the current VIA app.

This was written for a board that reports:

```text
Device: RDR EPOMAKER TH40
VID:    0x320F
PID:    0x5055
VIA:    protocol 9
```

## Problem

The TH40 can appear in Linux as a HID device and the VIA design JSON can load, but the Configure tab may fail with errors like:

```text
NotAllowedError: Failed to open the device.
Received invalid protocol version from device
Device: EPOMAKER TH40
Vid: 0x320F
Pid: 0x5055
```

In this case, the board answers VIA raw HID commands, but the current VIA app rejects the old protocol version. The EPOMAKER JSON I found also used different USB IDs:

```text
JSON VID: 0x36B0
JSON PID: 0x304E
```

while this keyboard actually reported:

```text
Actual VID: 0x320F
Actual PID: 0x5055
```

This tool bypasses the VIA GUI and talks to the keyboard over `/dev/hidraw`.

## What Works

This firmware has one important quirk:

- Reading keymap entries works through VIA command `0x12` (`dynamic_keymap_get_buffer`).
- Writing keymap entries works through VIA command `0x05` (`dynamic_keymap_set_keycode`).
- The usual per-key read command `0x04` returned protocol-looking data on my board, so this tool does not use it for normal `get` or `dump`.

## Build

```bash
cargo build --release
```

The binary will be:

```text
target/release/th40-via9
```

During development you can also run:

```bash
cargo run -- protocol
```

## Linux Access

You probably need a udev rule so a normal user session can access the TH40 hidraw device.

Create:

```text
/etc/udev/rules.d/92-epomaker-th40.rules
```

with:

```udev
KERNEL=="hidraw*", ATTRS{idVendor}=="320f", ATTRS{idProduct}=="5055", MODE="0660", TAG+="uaccess"
```

Then reload rules and reconnect the keyboard:

```bash
sudo udevadm control --reload-rules
sudo udevadm trigger --subsystem-match=hidraw
```

If permissions still fail, run commands with `sudo`.

## Usage

Default device path is `/dev/hidraw1`.

```bash
sudo ./target/release/th40-via9 protocol
sudo ./target/release/th40-via9 dump 4
sudo ./target/release/th40-via9 get <layer> <row> <col>
sudo ./target/release/th40-via9 set <layer> <row> <col> <key>
```

If your raw HID interface is different:

```bash
sudo ./target/release/th40-via9 /dev/hidraw2 protocol
```

## Backup

Before changing anything:

```bash
sudo ./target/release/th40-via9 dump 4 > th40-keymap-backup.txt
```

## Example: Programming Layer

My goal was to use right Ctrl as a custom programming-layer hold key, while keeping the stock Fn layer intact.

Commands:

```bash
sudo ./target/release/th40-via9 set 0 3 11 MO2
sudo ./target/release/th40-via9 set 2 0 9 LBRC
sudo ./target/release/th40-via9 set 2 0 10 RBRC
sudo ./target/release/th40-via9 set 2 1 8 SCLN
sudo ./target/release/th40-via9 set 2 1 9 QUOT
```

Result:

```text
RCtrl + O         -> [
RCtrl + Shift + O -> {
RCtrl + P         -> ]
RCtrl + Shift + P -> }

RCtrl + K         -> ;
RCtrl + Shift + K -> :

RCtrl + L         -> '
RCtrl + Shift + L -> "
```

## Key Names

Common key names supported by the tool:

```text
TAB ESC BSPC ENT SPC
A..Z 0..9
LBRC RBRC BSLS SCLN QUOT GRV COMM DOT SLASH MINS EQL
LEFT DOWN UP RIGHT HOME END PGUP PGDN DEL INS
LCTL LSFT LALT LGUI RCTL RSFT RALT RGUI
MO1 MO2 MO3 TO0 TO1 TO2 TO3
```

You can also pass raw QMK keycodes:

```bash
sudo ./target/release/th40-via9 set 2 0 9 0x002f
```
