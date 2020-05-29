package main

import (
	"fmt"
	"unsafe"

	wasm "github.com/wasmerio/go-ext-wasm/wasmer"
)

var store = map[string]string{}

func readDB(context unsafe.Pointer, keyPtr, keySize, valuePtr, valueSize, offset int32) int32 {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	realKey := memory[keyPtr: keyPtr + keySize]

	fmt.Printf("read key [%s]\n", string(realKey))

	var size int
	realValue := store[string(realKey)]
	size = len(realValue) // 如果key不存在 则size为0

	if offset >= int32(size) {
		return 0
	}

	index := offset + valueSize
	if index > int32(size) {
		index = int32(size)
	}

	copiedData := []byte(realValue)[offset: index]
	copy(memory[valuePtr:valuePtr+valueSize], copiedData)

	return int32(size)
}

func writeDB(context unsafe.Pointer, keyPtr, keySize, valuePtr, valueSize int32) {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	realKey := memory[keyPtr: keyPtr + keySize]
	realValue := memory[valuePtr: valuePtr + valueSize]

	fmt.Printf("write key [%s], value [%s]\n", string(realKey), string(realValue))

	store[string(realKey)] = string(realValue)
}

func deleteDB(context unsafe.Pointer, keyPtr, keySize int32) {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	realKey := memory[keyPtr: keyPtr + keySize]

	fmt.Printf("delete key [%s]\n", string(realKey))

	delete(store, string(realKey))
}
