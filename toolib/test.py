import time
from ctypes import cdll

lib = cdll.LoadLibrary("target/release/orz.dll")


t1 = time.time()
result = lib.add(35,35)
t2 = time.time()

print("====== Rust ======")
print("use time: " + str(round(t2 - t1, 2)) + "s")
print("result is: " + str(result))

print(result,type(result))


