## 设计

将传入的字符串与yyds拼接后MD5加密，然后返回加密后的字符串。
为了加密，将yyds按位转为16进制，然后与0x55进行按位异或计算后的到盐值。

## 编译

web 环境

`wasm-pack build --release --target web`

nodejs 环境

`wasm-pack build --release --target nodejs`
