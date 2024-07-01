# ASKME
askme是一个使用Rust编写的程序，通过调用CMD5 API实现密文反查的小工具。

支持两种查询模式:
```bash
askme.exe hash //单个查询
```
或stdin批量查询:
```cmd
C:/users/reg/desktop> askme.exe
[*] 输入要解密的哈希,以行分割,Ctrl+D 结束.
e6e061838856bf47e1de730719fb2609
7a4fcb0a976ee35dd4a3bd256373c266
21232f297a57a5a743894a0e4a801fc3
------------------------------

e6e061838856bf47e1de730719fb2609 admin@123
21232f297a57a5a743894a0e4a801fc3 admin
7a4fcb0a976ee35dd4a3bd256373c266 解密失败
```

**使用前需要在askme.conf中配置查询的用户凭据(首次运行程序会自动生成):**

```ini
email=EXAMPLE@qq.com
key=KEYKEYKEY
```

