# littleBrowser
# littleBrowser（Rust）

本目录下的 Rust 程序实现“任务1”要求的简易浏览器：

- URL 输入栏：输入 URL 并回车/点击 Go
- 构造并发送 HTTP 请求报文（TCP，HTTP/1.0，作业要求的 headers）
- 显示 HTTP 响应头与页面内容
- 解析并渲染基础标签：`title/h1-h3/p/br/a/img/pre`

## 运行

```bash
cd /Users/lazy/code/littleBrowser
make server-bg
make run
```

停止服务器：

```bash
make server-stop
```

## 输入示例

- 静态：`http://www.test.com.cn:8080/index.html`
- 动态：`http://www.test.com.cn:8080/adder.exe?1500&212`

更完整的本地静态/动态测试步骤见：`TESTING.md`。

## 中文显示成方块（口口）怎么办

这是因为默认字体不包含中文字形。项目启动时会优先尝试加载 `assets/fonts/` 下的 `.otf/.ttf` 中文字体。

- 放置说明：`assets/fonts/README.md`

## hosts 解析

程序按作业要求从 hosts 文件解析域名到 IP：

- macOS/Linux：`/etc/hosts`
- Windows：`C:\\Windows\\System32\\drivers\\etc\\hosts`

请先在 hosts 中添加 `www.test.com.cn` 对应的 IP。
