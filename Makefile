src/proto/mod.rs: src/proto/mod.proto
	protoc --rust_out $(dir $@) $^
