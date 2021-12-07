# Gigabit Communication Latency Tests

## Goal

The goal of this project is two fold.

1. Determine the real time capablilites of linux both with and without a real time patch using a variety of different scheduling calls to ensure that it can sustain a hard real time 10khz loop for GigaBIT's control system.
2. Determine the added latency that comes with using a distributed system where the sensor measuring computer and the motor commanding computer are different and simply share the necessary data over ethernet.

## Real Time Linux Patch Install

- Will begin this assessment by following the guidance of this [source](https://medium.com/@patdhlk/realtime-linux-e97628b51d5d).
- There is reason to believe that there may be benefit to cpu locking the process which can be done via `sched_setaffinity()` on Linux.

### On Raspberry Pi

- used `uname -r` found out usnig kernel 5.10.63-v71
- went to [real time patches website](https://cdn.kernel.org/pub/linux/kernel/projects/rt/) grabbed the nearest version 5.10.78

```bash
sudo wget https://cdn.kernel.org/pub/linux/kernel/projects/rt/5.10/patch-5.10.78-rt55.patch.xz
```

- ran config generator with `sudo make menuconfig`
- failed due to missing dependncies
- reran after installing dependcies
- went to kernel features->preemption model
- chose fully preemptible kernel (real-time)
- saved
- ran `sudo make -j4 && sudo make modules_install install `
- make install failed

```bash
sh ./arch/arm/boot/install.sh "5.10.78-rt55" \
arch/arm/boot/Image System.map "/boot"
run-parts: executing /etc/kernel/postinst.d/apt-auto-removal 5.10.78-rt55 /boot/vmlinuz-5.10.78-rt55
run-parts: executing /etc/kernel/postinst.d/initramfs-tools 5.10.78-rt55 /boot/vmlinuz-5.10.78-rt55
update-initramfs: Generating /boot/initrd.img-5.10.78-rt55
W: Possible missing firmware /lib/firmware/r8a779x_usb3_v3.dlmem for built-in driver xhci_plat_hcd
W: Possible missing firmware /lib/firmware/r8a779x_usb3_v2.dlmem for built-in driver xhci_plat_hcd
W: Possible missing firmware /lib/firmware/r8a779x_usb3_v1.dlmem for built-in driver xhci_plat_hcd
```

- will retry with an older kernel (4.9.47)
- running make failed with

```bash
  HOSTLD  scripts/dtc/dtc
/usr/bin/ld: scripts/dtc/dtc-parser.tab.o:(.bss+0x8): multiple definition of `yylloc'; scripts/dtc/dtc-lexer.lex.o:(.bss+0x1c): first defined here
collect2: error: ld returned 1 exit status
make[2]: *** [scripts/Makefile.host:116: scripts/dtc/dtc] Error 1
make[1]: *** [scripts/Makefile.build:544: scripts/dtc] Error 2
make[1]: *** Waiting for unfinished jobs....
  HOSTLD  scripts/mod/modpost
make: *** [Makefile:560: scripts] Error 2
```

- switched make file from `gcc` `gcc-9`
- making is now working for the older kernel

### Steps

- Got the Raspeberry pi imager for ubuntu installed [link](https://projects.raspberrypi.org/en/projects/raspberry-pi-setting-up/2)
- Wrote the raspeberry pi os (a debian fork) onto two sd cards

## To test

- Different sched methods
- with and without [RT patch](https://mirrors.edge.kernel.org/pub/linux/kernel/projects/rt/4.4/)
