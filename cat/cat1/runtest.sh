#!/bin/bash
set -e
gcc -o cat cat.c
diff <(./cat cat.c) cat.c
echo Done

