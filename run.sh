./build.sh

qemu-system-x86_64 -drive format=raw,file=bobos/bobos.bin -vnc :0
vncviewer localhost:5900
