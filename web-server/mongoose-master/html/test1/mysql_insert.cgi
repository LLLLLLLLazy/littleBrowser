#!/usr/bin/env python
# -*- coding: UTF-8 -*-
# CGI处理模块
import cgi, cgitb 
import MySQLdb

# 创建 FieldStorage 的实例化
form = cgi.FieldStorage() 
# 获取数据
first_name = form.getvalue('first_name')
last_name = form.getvalue('last_name')
age = form.getvalue('age')
search_result=""
# 打开数据库连接
db = MySQLdb.connect("localhost","root","111","TESTDB" )
# 使用cursor()方法获取操作游标 
cursor = db.cursor()
# SQL 查询语句
sql = """INSERT INTO EMPLOYEE(FIRST_NAME,
         LAST_NAME, AGE, SEX, INCOME)
         VALUES ('%s', '%s', '%s', 'F', 2110)"""%(first_name,last_name,age)


try:
   # 执行SQL语句
   cursor.execute(sql)
   # 获取所有记录列表
   db.commit()
   search_result='fname='+first_name+'  lname='+last_name+'   age='+age+'   sex=F'+'   income=2110'
except:
   print "Error: unable to fecth data"
# 关闭数据库连接
db.close()

#print search_result
print "Content-type:text/html"
print
print "<html>"
print "<head>"
print "<meta charset=\"utf-8\">"
print "<title>MYSQL CGI 测试实例</title>"
print "</head>"
print "<body>"
print "<h2>You have insert this tip: %s </h2>" % (search_result)
print "</body>"
print "</html>"
