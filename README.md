# rust-example
example of rust

# 说明
一个rust写的查询ip地址归属地的web程序

# 用法

请求：

http://127.0.0.1:8000/ipinfo?ip=1.0.33.0,1.100.32.0

应答：

`{
  "data": [
    {
      "ip": "1.0.33.0",
      "country": "中国",
      "area": "广东",
      "city": "*",
      "isp": "电信"
    }
  ],
  "count": 1,
  "code": 0,
  "msg": "ok"
}

