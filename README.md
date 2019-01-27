```sh
$ rustup component add llvm-tools-preview
$ ls -l /dev/bus/usb/001/002
$ getfacl /dev/bus/usb/001/002
$ openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```
