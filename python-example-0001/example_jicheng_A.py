# -*- coding: utf-8 -*-
"""
Created on Fri Sep  2 13:36:56 2022

@author: Userss
"""
from xml.etree.ElementTree import indent
from numpy import *
# import numpy
import json
import os


# --------------------参数输入---------------------------
n_layer = 8
L = [1e15, 1e15, 1e15, 1e15, 1e15, 1e15, 1e15]
Emm = [[9855, 12000, 11000, 9000, 13000, 13000, 720, 40.8]]
mu = [0.25, 0.25, 0.25, 0.40, 0.25, 0.25, 0.35, 0.40]
h = [0.04, 0.06, 0.08, 0.1, 0.18, 0.18, 0.20]
P = 0.7
Q = 0
a = 0.1065
p = 0.7
q = 0
xx_ = [0, 0]
yy_ = [-0.15975, 0.15975]
xx = [0.001, 0.001, 0.001, 0.001]
yy = [0.001, 0.026625, 0.054, 0.15975]
n_d = 50
upper_h = 400
n_i = 200
n_series = 1
zz = [0, 0.01, 0.04, 0.04, 0.28, 0.28]
NN = [1, 1, 1, 2, 4, 5]

# python创建字典dict
input_python = {
    'n_layer': n_layer,
    'L': L,
    'Emm': Emm,
    'mu': mu,
    'h': h,
    'P': P,
    'Q': Q,
    'a': a,
    'p': [p],
    'q': [q],
    'xx_': xx_,
    'yy_': yy_,
    'n_d': n_d,
    'upper_h': upper_h,
    'n_i': n_i,
    'n_series': n_series,
    'xx': xx,
    'yy': yy,
    'zz': zz,
    'NN': NN
}
print(type(input_python))

# python将字典dict转换为json字符串，添加indent参数格式更漂亮，且转换为json字符串之后会带上回车符（很重要）
c1 = json.dumps(input_python,indent=4)
print(type(c1))
print(c1)

# python将json字符串转换为字节串
c2 = c1.encode() 
print(type(c2))
print(c2)

# python将字节串转换为十六进制字符串
c3 = c2.hex()
print(type(c3))
print(c3)

# python将十六进制字符串转换为字节串
c4 = bytes.fromhex(c3)
print(c4)

# python将字节串转换为json字符串
c5 = c4.decode()
print(type(c5))
print(c5)

# python将json字符串转换为字典dict
c6 = json.loads(c5)
print(type(c6))
print(c6)






