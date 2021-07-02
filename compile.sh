#! /bin/bash
mkdir -p cpp
mkdir -p csharp
mkdir -p go
mkdir -p java
mkdir -p js
mkdir -p objc
mkdir -p php
mkdir -p python
mkdir -p ruby
protoc strawhat.proto --cpp_out=./cpp --csharp_out=./csharp --go_out=./go --java_out=./java --js_out=./js --objc_out=./objc --php_out=./php --python_out=./python --ruby_out=./ruby
