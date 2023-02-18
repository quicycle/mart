#! /usr/bin/env sh

touch sandwich-results.txt

./sandwiches.py -a 0 >> sandwich-results.txt
echo "" >> sandwich-results.txt
./sandwiches.py -a 123 >> sandwich-results.txt
echo "" >> sandwich-results.txt
./sandwiches.py -a 0123 >> sandwich-results.txt
echo "" >> sandwich-results.txt
./sandwiches.py -a 23 >> sandwich-results.txt
echo "" >> sandwich-results.txt
./sandwiches.py -a 01 >> sandwich-results.txt
echo "" >> sandwich-results.txt
./sandwiches.py -a 023 >> sandwich-results.txt
echo "" >> sandwich-results.txt
./sandwiches.py -a 1 >> sandwich-results.txt
