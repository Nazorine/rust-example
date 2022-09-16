# -*- coding: utf-8 -*-
"""
Created on Fri Sep  2 13:36:56 2022

@author: Userss
"""

import os
import subprocess  

# python中3种调用可执行文件.exe的方法
# 方法二、commands.getstatusoutput()  会保存可执行程序中的打印值和主函数的返回值，但不会将执行过程中要打印的内容打印出来
main = "rust-example5.exe"
if os.path.exists(main):  
    rc,out= subprocess.getstatusoutput(main+' '+'111')  
    print (rc)
    # print ('*'*10)
    print ("out=",out)
    print ("type of out=",type(out))




















