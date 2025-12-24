#!/usr/bin/python
# -*- coding: UTF-8 -*-
# CGI处理模块
import cgi, cgitb 
import MySQLdb

# 创建 FieldStorage 的实例化
form = cgi.FieldStorage() 
# 获取数据
user_name = form.getvalue('name')
search_result=""
# 打开数据库连接
db = MySQLdb.connect("localhost","root","111","TESTDB" )
# 使用cursor()方法获取操作游标 
cursor = db.cursor()
# SQL 查询语句
sql = "SELECT * FROM EMPLOYEE \
       WHERE FIRST_NAME ='%s' " % (user_name)
try:
   # 执行SQL语句
   cursor.execute(sql)
   # 获取所有记录列表
   results = cursor.fetchall()
   for row in results:
      fname = row[0]
      lname = row[1]
      age = row[2]
      sex = row[3]
      income = row[4]
      # 打印结果
      
      search_result='fname='+fname+'  lname='+lname+'   age='+str(age)+'   sex='+sex+'   income='+str(income)
      #print "fname=%s,lname=%s,age=%d,sex=%s,income=%d" % \
             #(fname, lname, age, sex, income )
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
print "<h2>Search_Result: %s </h2>" % (search_result)
print "</body>"
print "</html>"
