The problem of calculating oup enrichment from the knowning manufacturing parameters.
The problem is solved by sistem of nonlinear equations.
A well known python package scipy can do it in a simple and elengant way.
So we call a python function from rust, making a test and comparing benchmarks for rust and clear python.

============================= rust test session starts ===============================
bench_call_python_func ... bench:     170,305 ns/iter (+/- 36,730) or 5.87 Kops/s
									  
============================= python test session starts =============================
platform win32 -- Python 3.8.8, pytest-6.2.3, py-1.10.0, pluggy-0.13.1
benchmark: 3.4.1 (defaults: timer=time.perf_counter disable_gc=False min_rounds=5 min_time=0.000005 max_time=1.0 calibration_precision=10 warmup=False warmup_iterations=100000)

benchmark: 9 tests
|Name (time in ms)        |          Min         |        Max         |        Mean         |     StdDev         |     Median         |        IQR          |  Outliers|    OPS(Kops/s)  |     Rounds|  Iterations|
|-------------------------|----------------------|--------------------|---------------------|--------------------|--------------------|---------------------|----------|-----------------|-----------|------------|
|test_bench_calculate     |     150.0000         | 4,017.4000         |    178.3281         |   124.5243         |   156.9000         |     7.4000          |   213;391|    5.6076       |       3381|           1|

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean