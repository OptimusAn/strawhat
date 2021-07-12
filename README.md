# strawhat
tcp tunnel protocol

# 数据包部分字段简介

| 序号 | 名称   | 类型  | 说明  |
|  ----  |  ----  | ----  | ----  |
| 1 | ServiceID | string | serviceID |
| 2 | Transport | enum | Tcp:0/Udp:1 |

# go-code instruction

```
package main

import (
	"github.com/OptimusAn/strawhat/go/strawhat"
	"github.com/golang/protobuf/proto"
)

func main() {
	f := &strawhat.Strawhat{
		ServiceID: "baidu.com",
		Transport: strawhat.Transport_Tcp,
	}
	data, err := proto.Marshal(f)
	if err != nil {
		println(err)
		return
	}

	println("length:", len(data))

	newf := &strawhat.Strawhat{}
	if err := proto.Unmarshal(data, newf); err != nil {
		println(err)
		return
	}

	println("println:", newf.String())
}
```
