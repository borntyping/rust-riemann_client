#!/bin/bash

cd src/proto && protoc --rust_out . mod.proto
