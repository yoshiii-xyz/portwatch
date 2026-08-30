# portwatch

Watch TCP/UDP ports and show listening services.

## Install

```console
cargo build --release
sudo cp target/release/portwatch /usr/local/bin/
```

## Usage

```console
portwatch
portwatch --tcp
portwatch --udp
```

Output:

```
tcp   0.0.0.0:8080            0.0.0.0:*            LISTEN
```
