# -*- coding: utf-8 -*-
"""
Created on Fri Sep  2 13:36:56 2022

@author: Userss
"""
from numpy import *
# import numpy
import json
import os
import sys

# 在python程序中运行带参数的exe文件

main = r'rust-example5.exe' # 你要运行的exe文件
params1 = r'11' # 你要传入的参数
# 如果要传入多个参数，就继续定义，然后在os.system里加起来就行
print(main+' '+params1)

r_v = os.system(main+' '+params1)
print (r_v )




# print(sys.main,"11")

# 运行rust程序
# r_v = os.system(main)















