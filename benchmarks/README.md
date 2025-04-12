# Benchmarks

## Setup

To install the dependencies required for the benchmark, first run the command below.

```shell
cd nodegit
npm install
```

Because nodegit may not be installed depending on the environment (to be exact, it fails to build locally because there is no prebuilt), it is managed separately in this yarn workspace.

## Running

Run benchmarks with the below command:

```shell
just benchmarks
```

## Result

Tested with MacBook Pro (14-inch, M2 Pro, 2023).

```
  ✓ index.bench.ts > open 1815ms
     name                        hz     min      max    mean     p75     p99    p995    p999     rme  samples
   · es-git               11,590.57  0.0730  13.8912  0.0863  0.0863  0.1225  0.1327  0.2043  ±5.42%     5796
   · nodegit              11,813.89  0.0704   7.5226  0.0846  0.0835  0.1289  0.1521  0.2398  ±4.22%     5908   fastest
   · @napi-rs/simple-git   2,550.28  0.2557   4.3442  0.3921  0.4530  0.6906  0.7774  2.5094  ±2.52%     1276   slowest

 ✓ index.bench.ts > rev-parse 2426ms
     name                       hz      min      max     mean      p75      p99     p995     p999     rme  samples
   · es-git               6,356.94   0.1370  14.3807   0.1573   0.1550   0.2074   0.2238   0.2840  ±5.58%     3179   fastest
   · nodegit              5,688.24   0.1535  13.0313   0.1758   0.1749   0.2278   0.2493   0.3825  ±5.31%     2845
   · @napi-rs/simple-git  2,511.09   0.2856   1.0381   0.3982   0.4885   0.7190   0.7559   0.8119  ±1.75%     1256
   · child_process         88.5663  10.7419  12.6293  11.2910  11.3945  12.6293  12.6293  12.6293  ±1.08%       45   slowest

 ✓ index.bench.ts > revwalk 2448ms
     name                      hz      min      max     mean      p75      p99     p995     p999     rme  samples
   · es-git                841.62   1.1206  11.6329   1.1882   1.1721   1.4188   1.5288  11.6329  ±4.12%      421   fastest
   · nodegit               792.28   1.2102   1.8455   1.2622   1.2793   1.5123   1.7268   1.8455  ±0.46%      397
   · @napi-rs/simple-git   650.71   1.3304   2.1619   1.5368   1.6075   2.1102   2.1270   2.1619  ±1.42%      326
   · child_process        74.5720  12.9525  14.0355  13.4099  13.5443  14.0355  14.0355  14.0355  ±0.64%       38   slowest

 ✓ index.bench.ts > get commit 2451ms
     name                       hz      min      max     mean      p75      p99     p995     p999     rme  samples
   · es-git               7,628.10   0.1154   1.2045   0.1311   0.1328   0.1821   0.1898   0.3014  ±0.51%     3815   fastest
   · nodegit              6,723.92   0.1314  11.6206   0.1487   0.1474   0.2221   0.2447   0.3603  ±4.52%     3362
   · @napi-rs/simple-git  2,359.88   0.3026   0.7248   0.4238   0.5073   0.6490   0.6592   0.7231  ±1.42%     1180
   · child_process         78.9937  12.1418  14.0038  12.6592  12.7804  14.0038  14.0038  14.0038  ±0.96%       40   slowest
```

Summary:

```
  nodegit - index.bench.ts > open
    1.02x faster than es-git
    4.63x faster than @napi-rs/simple-git

  es-git - index.bench.ts > rev-parse
    1.12x faster than nodegit
    2.53x faster than @napi-rs/simple-git
    71.78x faster than child_process

  es-git - index.bench.ts > revwalk
    1.06x faster than nodegit
    1.29x faster than @napi-rs/simple-git
    11.29x faster than child_process

  es-git - index.bench.ts > get commit
    1.13x faster than nodegit
    3.23x faster than @napi-rs/simple-git
    96.57x faster than child_process
```
