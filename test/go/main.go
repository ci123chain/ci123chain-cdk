package main

// #include <stdlib.h>
//
// extern int read_db(void*, int, int, int, int, int);
// extern void write_db(void*, int, int, int, int);
// extern void delete_db(void*, int, int);
//
// extern int send(void*, int, long long);
// extern void get_creator(void*, int);
// extern void get_invoker(void*, int);
// extern long long get_time(void*);
//
// extern int get_input_length(void*, int);
// extern void get_input(void*, int, int, int);
// extern void notify_contract(void*, int, int);
// extern void return_contract(void*, int, int);
// extern int call_contract(void*, int, int, int);
import "C"
import (
	"fmt"
	"unicode/utf8"
	"unsafe"

	wasm "github.com/wasmerio/go-ext-wasm/wasmer"
)

//export read_db
func read_db(context unsafe.Pointer, keyPtr, keySize, valuePtr, valueSize, offset int32) int32 {
	return readDB(context, keyPtr, keySize, valuePtr, valueSize, offset)
}

//export write_db
func write_db(context unsafe.Pointer, keyPtr, keySize, valuePtr, valueSize int32) {
	writeDB(context, keyPtr, keySize, valuePtr, valueSize)
}

//export delete_db
func delete_db(context unsafe.Pointer, keyPtr, keySize int32) {
	deleteDB(context, keyPtr, keySize)
}

//export send
func send(context unsafe.Pointer, to int32, amount int64) int32 {
	return performSend(context, to, amount)
}

//export get_creator
func get_creator(context unsafe.Pointer, creatorPtr int32) {
	getCreator(context, creatorPtr)
}

//export get_invoker
func get_invoker(context unsafe.Pointer, invokerPtr int32) {
	getInvoker(context, invokerPtr)
}

//export get_time
func get_time(context unsafe.Pointer) int64 {
	return getTime(context)
}

//export get_input_length
func get_input_length(context unsafe.Pointer, token int32) int32 {
	return getInputLength(context, token)
}

//export get_input
func get_input(context unsafe.Pointer, token, ptr, size int32) {
	getInput(context, token, ptr, size)
}

//export notify_contract
func notify_contract(context unsafe.Pointer, ptr, size int32) {
	notifyContract(context, ptr, size)
}

//export return_contract
func return_contract(context unsafe.Pointer, ptr, size int32) {
	returnContract(context, ptr, size)
}

//export call_contract
func call_contract(context unsafe.Pointer, addrPtr, inputPtr, inputSize int32) int32 {
	return callContract(context, addrPtr, inputPtr, inputSize)
}

var inputData = map[int32][]byte{}

func getBytes() []byte {
	modulePath := "../../example/target/example.wasm"

	res, err := wasm.ReadBytes(modulePath)
	if err != nil {
		panic(err)
	}

	return res
}

func ontologyContract() {
	imports, err := wasm.NewImports().Namespace("env").Append("send", send, C.send)
	if err != nil {
		panic(err)
	}

	_, _ = imports.Append("read_db", read_db, C.read_db)
	_, _ = imports.Append("write_db", write_db, C.write_db)
	_, _ = imports.Append("delete_db", delete_db, C.delete_db)

	_, _ = imports.Append("get_creator", get_creator, C.get_creator)
	_, _ = imports.Append("get_invoker", get_invoker, C.get_invoker)
	_, _ = imports.Append("get_time", get_time, C.get_time)

	_, _ = imports.Append("get_input_length", get_input_length, C.get_input_length)
	_, _ = imports.Append("get_input", get_input, C.get_input)
	_, _ = imports.Append("return_contract", return_contract, C.return_contract)
	_, _ = imports.Append("notify_contract", notify_contract, C.notify_contract)
	_, _ = imports.Append("call_contract", call_contract, C.call_contract)

	module, err := wasm.Compile(getBytes())
	if err != nil {
		panic(err)
	}
	defer module.Close()

	instance, err := module.InstantiateWithImports(imports)
	if err != nil {
		panic(err)
	}
	defer instance.Close()

	invoke, exist := instance.Exports["invoke"]
	if !exist {
		fmt.Println(exist)
		return
	}

	params := [][]interface{}{
		{"write_db", "time", "机器"},
		{"read_db", "time"},
		{"delete_db", "time"},
		{"send", NewAddress([]byte("user0000000000000000")), uint64(7)},
		{"get_creator"},
		{"get_invoker"},
		{"get_time"},
		{"call_contract", NewAddress([]byte("contract000000000000")), uint32(3), []byte{1, 2, 3}},
		{"notify"},
		{"这是一个无效的方法"},
	}

	for _, param := range params {
		fmt.Printf("\n==============================\ncall %s\n", param[0])
		inputData[0] = serialize(param)
		_, err = invoke()
		if err != nil {
			panic(err)
		}
	}
}

func serialize(raw []interface{}) (res []byte) {
	sink := NewSink(res)

	for i := range raw {
		switch r := raw[i].(type) {
		case string:
			//字符串必须是合法的utf8字符串
			if !utf8.ValidString(r) {
				panic("invalid utf8 string")
			}
			sink.WriteString(r)

		case uint64:
			sink.WriteU64(r)

		case uint32:
			sink.WriteU32(r)

		case []byte:
			sink.WriteBytes(r)

		case Address:
			sink.WriteBytes(r[:])

		default:
			panic("unexpected type")
		}
	}

	return sink.Bytes()
}

func main() {
	ontologyContract()
}
