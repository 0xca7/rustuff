# Process Injector

All credit goes to the author of this paper:
https://papers.vx-underground.org/papers/VXUG/Mirrors/Injection/linux/0x00sec.org-Linux%20Infecting%20Running%20Processes.pdf

I just wrote this in rust for fun and to get more experience with rust.

## Running

This injects shellcode, which prints "Hello World", into the executable in the dummy folder.

```
cd dummy
gcc main.c -o main
cd ..
cargo build
cp target/debug/injector .

# in terminal 1 run dummy executable which will print it's PID
cd dummy
./main

# run the injector with the PID
sudo ./injector [PID]
```

After injection dummy will print "Hello World" and stop. 

---

#### 0xca7

