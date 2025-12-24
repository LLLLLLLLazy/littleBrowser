#!/usr/bin/env python3
# -*- coding: UTF-8 -*-

import cgi

form = cgi.FieldStorage()

site_name = form.getvalue('name') or ''
site_url = form.getvalue('url') or ''

print("Content-type:text/html")
print()
print("<html>")
print("<head>")
print("<meta charset=\"utf-8\">")
print("<title>GET_POST方法 CGI 测试实例</title>")
print("</head>")
print("<body>")
print(f"<h2>{site_name}MyNet：{site_url}</h2>")
print("</body>")
print("</html>")
