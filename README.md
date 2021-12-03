# Gigabit Communication Latency Tests

## Goal

The goal of this project is two fold.

1. Determine the real time capablilites of linux both with and without a real time patch using a variety of different scheduling calls to ensure that it can sustain a hard real time 10khz loop for GigaBIT's control system.
2. Determine the added latency that comes with using a distributed system where the sensor measuring computer and the motor commanding computer are different and simply share the necessary data over ethernet.

## Real Time Linux

- Will begin this assessment by following the guidance of this [source](https://medium.com/@patdhlk/realtime-linux-e97628b51d5d).
- There is reason to believe that there may be benefit to cpu locking the process which can be done via `sched_setaffinity()` on Linux.

### Steps

- Got the Raspeberry pi imager for ubuntu installed [link](https://projects.raspberrypi.org/en/projects/raspberry-pi-setting-up/2)
- Wrote the raspeberry pi os (a debian fork) onto two sd cards

## To test

- Different sched methods
- with and without [RT patch](https://mirrors.edge.kernel.org/pub/linux/kernel/projects/rt/4.4/)
