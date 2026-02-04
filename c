dir=$(ls -dt */ | head -n 1)
file=$(ls -t $dir | grep '\.cpp' | head -n 1)
g++ $dir$file -o Main -O2 -lm -static -std=gnu++17
./Main.exe < stdin.txt
