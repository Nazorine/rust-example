# -*- coding: utf-8 -*-
"""
Created on Fri Sep  2 13:36:56 2022

@author: Userss
"""

import os

# python中3种调用可执行文件.exe的方法
# 方法三、popen()  会保存可执行程序中的打印值，但不会保存主函数的返回值，也但不会将执行过程中要打印的内容打印出来
main = "rust-example5.exe"
f = os.popen(main+' '+'111')    
data = f.readlines()    
f.close()    
print (data)



















