# About CPU statistics

The output of the built-in top utility looks like this (+ we use the -i flag to hide zero processes)

```
Tasks: 264 total, 1 running, 263 sleep, 0 d-sleep, 0 stopped, 0 zombie
%Cpu(s):  1.5 us,  0.7 sy,  0.0 ni, 97.5 id,  0.0 wa,  0.3 hi,  0.0 si,  0.0 st 
top - 22:01:08 up 45 min,  1 user,  load average: 0.35, 0.37, 0.41
Tasks: 264 total, 1 running, 263 sleep, 0 d-sleep, 0 stopped, 0 zombie
%Cpu(s):  1.3 us,  0.6 sy,  0.0 ni, 97.6 id,  0.0 wa,  0.2 hi,  0.2 si,  0.0 st 
MiB Mem :  15844.9 total,   8542.3 free,   4077.8 used,   3773.6 buff/cache     
MiB Swap:   7922.0 total,   7922.0 free,      0.0 used.  11767.1 avail Mem 

    PID USER      PR  NI    VIRT    RES    SHR S  %CPU  %MEM     TIME+ COMMAND                                                                                                                                   
   8268 theun1c   20   0 5475840 533660 363460 S   4.3   3.3   0:37.87 spotify                                                                                                                                   
   1140 theun1c    9 -11  181084  21748   9912 S   1.7   0.1   0:12.00 pipewire-pulse                                                                                                                            
   1764 theun1c   20   0 1224.4g 152968 117456 S   1.0   0.9   0:08.87 yandex_browser                                                                                                                            
   4985 theun1c   20   0  372668  75204  16904 S   1.0   0.5   0:31.70 nvim                                                                                                                                      
   8408 theun1c   20   0 1395.1g 412320 148228 S   1.0   2.5   0:20.66 spotify                                                                                                                                   
   8325 theun1c   20   0   34.4g 276216 214752 S   0.7   1.7   0:04.00 spotify                                                                                                                                   
   8750 theun1c   20   0 1226.1g 281740 169312 S   0.7   1.7   0:20.95 yandex_browser                                                                                                                            
    114 root      20   0       0      0      0 I   0.3   0.0   0:19.01 kworker/u32:3-ext4-rsv-conversion                                                                                                         
    901 theun1c   -2   0 2429008 368280 299260 S   0.3   2.3   2:17.06 kwin_wayland                                                                                                                              
    973 theun1c    9 -11  107804  19608  10048 S   0.3   0.1   0:02.93 pipewire                                                                                                                                  
   1334 theun1c   20   0   33.4g 617024 425380 S   0.3   3.8   2:27.90 yandex_browser                                                                                                                            
   1555 theun1c   20   0   33.3g 325252 225116 S   0.3   2.0   1:48.07 yandex_browser                                                                                                                            
   1563 theun1c   20   0   32.4g 144052 111032 S   0.3   0.9   0:24.95 yandex_browser                                                                                                                            
   4966 theun1c   20   0 1160908 277888 249328 S   0.3   1.7   0:23.42 konsole                                                                                                                                   
   8204 root      20   0       0      0      0 I   0.3   0.0   0:00.73 kworker/6:2-events                                                                                                                        
   8659 theun1c   20   0   11088   7936   5792 S   0.3   0.0   0:01.67 top                                                                                                                                       
   9832 theun1c   20   0   11088   7876   5740 R   0.3   0.0   0:00.02 top                

```

To understand cpu statistics, you can read this article on [Habr](https://habr.com/ru/articles/876428/)

```
# a small conclusion of CPU statistics

Load average: 6.17, 5.91, 5.64. Load over the last 1, 5, and 15 minutes. If the machine has 8 cores, for example, these values are not critical. It's helpful to remember that Load Average includes both CPU-using processes and those waiting for I/O (IO wait). 

us = 7.6% - user processes are not particularly overloaded.

sy = 5.7% — moderate load on the core.

id = 55.1% — more than half of the CPU time is idle.

wa = 26.3% — quite high I/O waiting. This may indicate intensive writing/reading. In this case, it is likely that Redis is dumping its data. For now, we assume that we do not know this for sure. However, we will reveal this information throughout the series of articles.

si = 5.2% — there is some soft interrupt load.

st = 0.0% — there is no "stolen" time on this instance
```


I am going to use ``` top -i | grep %Cpu ``` command with highlighting the stats 
