## Bare metal rust

##### vm

```bash
    qemu-system-x86_64 -drive format=raw,file=target/AMD64/debug/bootimage-os.bin
```

##### usb

```bash
    dd if=target/AMD64/debug/bootimage-os.bin of=/dev/*** && sync
```

#### cargo

```bash 
    cargo run
```
