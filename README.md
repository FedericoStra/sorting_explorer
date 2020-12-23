Sorting Explorer
================

An exploration of various sorting algorithms.

It is inspired by the similar project
[https://github.com/FedericoStra/Sorting](https://github.com/FedericoStra/Sorting),
which compares the performance of various sorting algorightms written in `C`.

Current status
--------------

This project is a work in progress.

The foundation and the structure of the library are laid out, we just need more
algorithms need to be implemented.

Benchmarks
----------

```
Reversed/BubbleSort/0   time:   [418.69 ps 429.62 ps 438.51 ps]                                   
Reversed/InsertionSort/0                                                                             
                        time:   [1.7314 ns 1.7364 ns 1.7429 ns]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe
Reversed/InsertionSort (binary)/0                                                                             
                        time:   [2.7194 ns 2.7226 ns 2.7265 ns]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
Reversed/BubbleSort/1   time:   [352.60 ps 354.41 ps 356.52 ps]                                   
Found 13 outliers among 100 measurements (13.00%)
  3 (3.00%) high mild
  10 (10.00%) high severe
Reversed/InsertionSort/1                                                                             
                        time:   [1.7601 ns 1.7654 ns 1.7717 ns]
Found 15 outliers among 100 measurements (15.00%)
  4 (4.00%) high mild
  11 (11.00%) high severe
Reversed/InsertionSort (binary)/1                                                                             
                        time:   [2.7250 ns 2.7296 ns 2.7350 ns]
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) high mild
  9 (9.00%) high severe
Reversed/BubbleSort/5   time:   [18.763 ns 18.794 ns 18.828 ns]                                  
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
Reversed/InsertionSort/5                                                                            
                        time:   [9.2245 ns 9.2400 ns 9.2579 ns]
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) high mild
  9 (9.00%) high severe
Reversed/InsertionSort (binary)/5                                                                            
                        time:   [26.410 ns 26.440 ns 26.472 ns]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
Reversed/BubbleSort/10  time:   [69.977 ns 70.080 ns 70.194 ns]                                   
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
Reversed/InsertionSort/10                                                                            
                        time:   [34.708 ns 34.776 ns 34.854 ns]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
Reversed/InsertionSort (binary)/10                                                                            
                        time:   [69.129 ns 69.236 ns 69.353 ns]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
Reversed/BubbleSort/50  time:   [2.5240 us 2.5291 us 2.5343 us]                                    
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
Reversed/InsertionSort/50                                                                             
                        time:   [1.0507 us 1.0526 us 1.0546 us]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
Reversed/InsertionSort (binary)/50                                                                             
                        time:   [582.31 ns 583.75 ns 585.43 ns]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
Reversed/BubbleSort/100 time:   [10.271 us 10.289 us 10.307 us]                                     
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) low severe
  1 (1.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe
Reversed/InsertionSort/100                                                                             
                        time:   [4.5992 us 4.6088 us 4.6190 us]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
Reversed/InsertionSort (binary)/100                                                                             
                        time:   [1.3299 us 1.3332 us 1.3369 us]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
Reversed/BubbleSort/500 time:   [210.42 us 210.76 us 211.14 us]                                    
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
Reversed/InsertionSort/500                                                                            
                        time:   [129.03 us 129.25 us 129.49 us]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
Reversed/InsertionSort (binary)/500                                                                             
                        time:   [10.468 us 10.501 us 10.541 us]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) low mild
  1 (1.00%) high mild
Reversed/BubbleSort/1000                                                                            
                        time:   [826.80 us 827.57 us 828.39 us]
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe
Reversed/InsertionSort/1000                                                                            
                        time:   [526.41 us 527.36 us 528.36 us]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
Reversed/InsertionSort (binary)/1000                                                                             
                        time:   [31.909 us 32.089 us 32.298 us]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
Reversed/BubbleSort/5000                                                                            
                        time:   [20.561 ms 20.592 ms 20.624 ms]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
Reversed/InsertionSort/5000                                                                            
                        time:   [13.364 ms 13.385 ms 13.407 ms]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
Reversed/InsertionSort (binary)/5000                                                                            
                        time:   [456.45 us 457.13 us 457.87 us]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

Shuffled/BubbleSort/0   time:   [518.44 ps 521.01 ps 523.32 ps]                                   
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
Shuffled/InsertionSort/0                                                                             
                        time:   [2.2244 ns 2.2288 ns 2.2337 ns]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
Shuffled/InsertionSort (binary)/0                                                                             
                        time:   [3.0908 ns 3.0977 ns 3.1050 ns]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
Shuffled/BubbleSort/1   time:   [516.22 ps 518.94 ps 522.09 ps]                                   
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
Shuffled/InsertionSort/1                                                                             
                        time:   [2.3077 ns 2.3147 ns 2.3226 ns]
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe
Shuffled/InsertionSort (binary)/1                                                                             
                        time:   [3.0983 ns 3.1121 ns 3.1268 ns]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe
Shuffled/BubbleSort/5   time:   [10.127 ns 10.171 ns 10.216 ns]                                  
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe
Shuffled/InsertionSort/5                                                                             
                        time:   [6.6989 ns 6.7123 ns 6.7283 ns]
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe
Shuffled/InsertionSort (binary)/5                                                                            
                        time:   [23.204 ns 23.250 ns 23.295 ns]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
Shuffled/BubbleSort/10  time:   [44.649 ns 44.709 ns 44.775 ns]                                   
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe
Shuffled/InsertionSort/10                                                                            
                        time:   [21.597 ns 21.643 ns 21.694 ns]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe
Shuffled/InsertionSort (binary)/10                                                                            
                        time:   [56.164 ns 56.270 ns 56.381 ns]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
Shuffled/BubbleSort/50  time:   [1.4138 us 1.4171 us 1.4206 us]                                    
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe
Shuffled/InsertionSort/50                                                                             
                        time:   [532.14 ns 533.22 ns 534.57 ns]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe
Shuffled/InsertionSort (binary)/50                                                                             
                        time:   [498.14 ns 499.90 ns 501.71 ns]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
Shuffled/BubbleSort/100 time:   [7.6669 us 7.6861 us 7.7064 us]                                     
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe
Shuffled/InsertionSort/100                                                                             
                        time:   [2.7545 us 2.7602 us 2.7662 us]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
Shuffled/InsertionSort (binary)/100                                                                             
                        time:   [1.1876 us 1.1901 us 1.1925 us]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
Shuffled/BubbleSort/500 time:   [286.42 us 286.82 us 287.23 us]                                    
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
Shuffled/InsertionSort/500                                                                            
                        time:   [68.886 us 69.031 us 69.189 us]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
Shuffled/InsertionSort (binary)/500                                                                             
                        time:   [20.203 us 20.239 us 20.279 us]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
Shuffled/BubbleSort/1000                                                                            
                        time:   [1.1025 ms 1.1034 ms 1.1044 ms]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe
Shuffled/InsertionSort/1000                                                                            
                        time:   [274.96 us 275.45 us 275.95 us]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
Shuffled/InsertionSort (binary)/1000                                                                            
                        time:   [53.588 us 53.687 us 53.787 us]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
Shuffled/BubbleSort/5000                                                                            
                        time:   [31.427 ms 31.475 ms 31.527 ms]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
Shuffled/InsertionSort/5000                                                                            
                        time:   [6.7760 ms 6.7862 ms 6.7970 ms]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
Shuffled/InsertionSort (binary)/5000                                                                            
                        time:   [465.02 us 465.60 us 466.20 us]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

```
