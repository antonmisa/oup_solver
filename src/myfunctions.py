from scipy.optimize import fsolve
from math import log

def equations(p, *args):
    x, y = p
    u_src, oup, epp, uf6 = args
    return (oup*((2*x/100-1)*log(x/(100-x))-(2*y/100-1)*log(y/(100-y)))-uf6*((2*u_src/100-1)*log(u_src/(100-u_src))-(2*y/100-1)*log(y/(100-y))) - epp, oup*(x-y)/(u_src-y) - uf6)

def calculate(u_oup, u_xv, u_src, oup, epp, uf6):
    x, y =  fsolve(equations, x0=(u_oup, u_xv), args=(u_src, oup, epp, uf6))
    return x, y

#Benchmarked values for pytest (in a single file just for simplicity)

def test_bench_calculate(benchmark):
    u_oup = 4.9
    u_xv = 0.13
    u_src = 0.215917
    oup = 6085.116
    epp = 150861.133
    uf6 = 338032.351
    data = (u_oup, u_xv, u_src, oup, epp, uf6)
    benchmark(calculate, *data)

def test_calculate():
    u_oup = 4.9
    u_xv = 0.13
    u_src = 0.215917
    oup = 6085.116
    epp = 150861.133
    uf6 = 338032.351
    x, y = calculate(u_oup, u_xv, u_src, oup, epp, uf6)
    assert (x, y) == (4.902748029373128, 0.13000000031162223)
