package main

// #include <stdlib.h>
//
// extern int println_str(void *context, int offset, int length);
// extern int read_db(void *context, int x, int y);
// extern void write_db(void *context, int x, int y);
// extern void delete_db(void *context, int x);
import "C"
import (
	"bytes"
	"encoding/binary"
	"fmt"
	wasm "github.com/wasmerio/go-ext-wasm/wasmer"
	"unsafe"
)

//export println_str
func println_str(context unsafe.Pointer, offset, length int32) int32 {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()
	fmt.Println(string(memory[offset : offset+length]))
	return length
}

//export read_db
func read_db(context unsafe.Pointer, key, value int32) int32 {
	fmt.Println("read")

	store := map[string]string{"testget": "1422"}

	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()
	keyAddr := NewRegion(memory[key : key+12])
	fmt.Println(keyAddr)
	realKey := memory[keyAddr.Offset : keyAddr.Offset+keyAddr.Length]
	fmt.Println(string(realKey))
	fmt.Println()

	allocate, exist := middleIns.fun["allocate"]
	if !exist {
		panic("allocate not found")
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
	copy(memory[value:value+12], region.ToBytes())

	return 0
}

//export write_db
func write_db(context unsafe.Pointer, key, value int32) {}

//export delete_db
func delete_db(context unsafe.Pointer, key int32) {}

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

type middle struct {
	fun map[string]func(...interface{}) (wasm.Value, error)
}

var middleIns = middle{fun: make(map[string]func(...interface{}) (wasm.Value, error))}

func GetBytes() []byte {
	modulePath := "/home/thomas/code/repo_77af9c18e026464fb8aad1444645db73/target/wasm32-unknown-unknown/debug/rust_sdk.wasm"

	bytes, _ := wasm.ReadBytes(modulePath)

	return bytes
}

func exportedFunctions() {
	// Instantiates a WebAssembly instance from bytes.
	instance, _ := wasm.NewInstance(GetBytes())
	defer instance.Close()

	for k, _ := range instance.Exports {
		fmt.Println(k)
	}

	// Gets an exported function.
	init, functionExists := instance.Exports["init"]

	fmt.Println(functionExists)
	if !functionExists {
		return
	}

	// Calls the `sum` exported function with Go values.
	result, _ := init(1)

	fmt.Println(result)

	var bytes []byte
	data := instance.Memory.Data()[result.ToI32():]
	for i := range data {
		if data[i] == 0 {
			break
		}
		bytes = append(bytes, data[i])
	}
	fmt.Println(string(bytes))

	//result, _ = init(wasm.I32(3))
	//
	//// Calls the `sum` exported function with WebAssembly values.
	//fmt.Println(result)

	// Output:
	// true
	// 3
	// 7
}

func dbFunc() {
	imports, err := wasm.NewImports().Namespace("env").Append("read_db", read_db, C.read_db)
	if err != nil {
		panic(err)
	}

	imports.Namespace("env").Append("write_db", write_db, C.write_db)
	imports.Namespace("env").Append("delete_db", delete_db, C.delete_db)

	module, err := wasm.Compile(GetBytes())
	if err != nil {
		panic(err)
	}
	defer module.Close()

	instance, err := module.InstantiateWithImports(imports)
	if err != nil {
		panic(err)
	}
	defer instance.Close()

	allocate, exist := instance.Exports["allocate"]
	if !exist {
		fmt.Println(exist)
		return
	}
	middleIns.fun["allocate"] = allocate

	init, exist := instance.Exports["init"]
	if !exist {
		fmt.Println(exist)
		return
	}

	offset, err := allocate(3)
	if err != nil {
		panic(err)
	}
	copy(instance.Memory.Data()[offset.ToI32():offset.ToI32()+3], "get")

	res, err := init(offset)
	if err != nil {
		panic(err)
	}
	var bytes []byte
	data := instance.Memory.Data()[res.ToI32():]
	for i := range data {
		if data[i] == 0 {
			break
		}
		bytes = append(bytes, data[i])
	}
	fmt.Println(string(bytes))
}

func main() {
	dbFunc()
}

func importedFunc() {
	imports, err := wasm.NewImports().Namespace("env").Append("println_str", println_str, C.println_str)
	if err != nil {
		panic(err)
	}

	module, err := wasm.Compile(GetBytes())
	if err != nil {
		panic(err)
	}
	defer module.Close()

	instance, err := module.InstantiateWithImports(imports)
	if err != nil {
		panic(err)
	}
	defer instance.Close()

	convert, exist := instance.Exports["convert"]
	if !exist {
		fmt.Println(exist)
		return
	}

	result, err := convert(30)
	if err != nil {
		panic(err)
	}

	fmt.Println(result)
}
