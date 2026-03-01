dir=$(ls -dt */ | head -n 1)
file=$(ls -t $dir | grep '\.cpp' | head -n 1)
g++ $dir$file -o Main -std=gnu++17
program=$(ls Main* | head -n 1)
chmod +x $program
TIMEFORMAT=%Ss
{ time ./$program < stdin.txt; } 2> time.txt
echo
cat time.txt | tr -d '\n'
