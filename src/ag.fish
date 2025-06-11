#!/usr/bin/fish
#run := ./ag.fish

cargo build --release
echo "gshare fp_1"
bunzip2 -kc ../traces/fp_1.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/release/bp --gshare 13
echo "gshare fp_2"
bunzip2 -kc ../traces/fp_2.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/release/bp --gshare 13
echo "gshare int_1"
bunzip2 -kc ../traces/int_1.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/release/bp --gshare 13
echo "gshare int_2"
bunzip2 -kc ../traces/int_2.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/release/bp --gshare 13
echo "gshare mm_1.bz2"
bunzip2 -kc ../traces/mm_1.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/release/bp --gshare 13
echo "gshare mm_2.bz2"
bunzip2 -kc ../traces/mm_2.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/release/bp --gshare 13

echo ""
echo "tournament fp_1"
bunzip2 -kc ../traces/fp_1.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/release/bp --tournament 9 10 10
echo "tournament fp_2"
bunzip2 -kc ../traces/fp_2.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/release/bp --tournament 9 10 10
echo "tournament int_1"
bunzip2 -kc ../traces/int_1.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/release/bp --tournament 9 10 10
echo "tournament int_2"
bunzip2 -kc ../traces/int_2.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/release/bp --tournament 9 10 10
echo "tournament mm_1.bz2"
bunzip2 -kc ../traces/mm_1.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/release/bp --tournament 9 10 10
echo "tournament mm_2.bz2"
bunzip2 -kc ../traces/mm_2.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/release/bp --tournament 9 10 10

echo ""
echo "custom fp_1"
bunzip2 -kc ../traces/fp_1.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/release/bp --custom
echo "custom fp_2"
bunzip2 -kc ../traces/fp_2.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/release/bp --custom
echo "custom int_1"
bunzip2 -kc ../traces/int_1.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/release/bp --custom
echo "custom int_2"
bunzip2 -kc ../traces/int_2.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/release/bp --custom
echo "custom mm_1.bz2"
bunzip2 -kc ../traces/mm_1.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/release/bp --custom
echo "custom mm_2.bz2"
bunzip2 -kc ../traces/mm_2.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/release/bp --custom
