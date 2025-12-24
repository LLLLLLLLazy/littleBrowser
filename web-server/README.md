# web-server（mongoose）使用说明

这个目录里的 `mongoose-master` 是服务器端，用来给你的 Rust littleBrowser 提供静态页面与 CGI 动态页面。

## 1) 编译服务器

macOS：

```bash
cd /Users/lazy/code/littleBrowser/web-server/mongoose-master
make mac
```

Linux：

```bash
cd /Users/lazy/code/littleBrowser/web-server/mongoose-master
make linux
```

编译产物：当前目录下的 `./mongoose`。

## 2) 准备 CGI 脚本可执行权限

首次使用建议执行（否则可能 403/500 或无法执行）：

```bash
cd /Users/lazy/code/littleBrowser/web-server/mongoose-master
chmod +x html/cgi-bin/*.cgi html/test1/*.cgi
```

说明：仓库里部分 `.cgi` 原本是 Python2 写法，我已将 `hello.cgi` 与 `test1/GET_POST.cgi` 改成 Python3 可运行。

## 3) 启动服务器

```bash
cd /Users/lazy/code/littleBrowser/web-server/mongoose-master
./mongoose
```

默认：监听 `8080`，网站根目录为 `mongoose-master/html`。

检查是否监听成功：

```bash
lsof -nP -iTCP:8080 -sTCP:LISTEN
```

## 4) 用 curl 验证

静态页面：

```bash
curl -v http://127.0.0.1:8080/test1/GET_POST.html
```

CGI：

```bash
curl -v "http://127.0.0.1:8080/cgi-bin/hello.cgi"
curl -v "http://127.0.0.1:8080/test1/GET_POST.cgi?name=abc&url=xyz"
```

## 5) 用 littleBrowser 访问

1. hosts 映射（按作业要求用 hosts 解析域名）：

```bash
sudo sh -c 'printf "\n127.0.0.1 www.test.com.cn\n" >> /etc/hosts'
```

2. 在 littleBrowser 输入：

- 静态：`http://www.test.com.cn:8080/test1/GET_POST.html`
- 动态（CGI）：`http://www.test.com.cn:8080/cgi-bin/hello.cgi`
- 动态（带参数）：`http://www.test.com.cn:8080/test1/GET_POST.cgi?name=abc&url=xyz`

如果看到 `Connection refused`：说明服务器没启动或 8080 端口没监听。
