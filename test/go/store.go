package main

import (
	"bytes"
	"encoding/binary"
	"fmt"
	wasm "github.com/wasmerio/go-ext-wasm/wasmer"
	"unsafe"
)

const RegionSize = 12

var store = map[string]string{}

//export read_db
func readDB(context unsafe.Pointer, key, value int32) int32 {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()
	keyAddr := NewRegion(memory[key : key+RegionSize])
	realKey := memory[keyAddr.Offset : keyAddr.Offset+keyAddr.Length]

	fmt.Printf("read key [%s]\n", string(realKey))

	allocate, exist := middleIns.fun["allocate"]
	if !exist {
		panic("allocate not found")
	}

	if _, exist := store[string(realKey)]; !exist {
		panic(string(realKey) + " not found")
	}

	size := len(store[string(realKey)])

	valueOffset, err := allocate(size)
	if err != nil {
		panic(err)
	}
	copy(memory[valueOffset.ToI32():valueOffset.ToI32()+int32(size)], store[string(realKey)])

	region := Region{
		Offset:   uint32(valueOffset.ToI32()),
		Capacity: uint32(size),
		Length:   uint32(size),
	}
	copy(memory[value:value+RegionSize], region.ToBytes())

	return 0
}

//export write_db
func writeDB(context unsafe.Pointer, key, value int32) {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()
	keyAddr := NewRegion(memory[key : key+RegionSize])
	realKey := memory[keyAddr.Offset : keyAddr.Offset+keyAddr.Length]
	valueAddr := NewRegion(memory[value : value+RegionSize])
	realValue := memory[valueAddr.Offset : valueAddr.Offset+valueAddr.Length]

	fmt.Printf("write key [%s], value [%s]\n", string(realKey), string(realValue))

	store[string(realKey)] = string(realValue)
}

//export delete_db
func deleteDB(context unsafe.Pointer, key int32) {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()
	keyAddr := NewRegion(memory[key : key+RegionSize])
	realKey := memory[keyAddr.Offset : keyAddr.Offset+keyAddr.Length]

	fmt.Printf("delete key [%s]\n", string(realKey))

	delete(store, string(realKey))
}

//Region 内存指针
type Region struct {
	Offset   uint32
	Capacity uint32
	Length   uint32
}

func NewRegion(b []byte) Region {
	var ret Region
	bytesBuffer := bytes.NewBuffer(b)
	_ = binary.Read(bytesBuffer, binary.LittleEndian, &ret.Offset)
	_ = binary.Read(bytesBuffer, binary.LittleEndian, &ret.Capacity)
	_ = binary.Read(bytesBuffer, binary.LittleEndian, &ret.Length)
	return ret
}

func (region Region) ToBytes() []byte {
	bytesBuffer := bytes.NewBuffer([]byte{})
	_ = binary.Write(bytesBuffer, binary.LittleEndian, region.Offset)
	_ = binary.Write(bytesBuffer, binary.LittleEndian, region.Capacity)
	_ = binary.Write(bytesBuffer, binary.LittleEndian, region.Length)
	return bytesBuffer.Bytes()
}