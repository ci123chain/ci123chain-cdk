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
// extern void destroy_contract(void*);
// extern int migrate_contract(void*, int, int, int, int, int, int, int, int, int, int, int, int, int);
// extern void panic_contract(void*, int, int);
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

//export migrate_contract
func migrate_contract(context unsafe.Pointer, codePtr, codeSize, namePtr, nameSize, verPtr, verSize,
	authorPtr, authorSize, emailPtr, emailSize, descPtr, descSize, newAddrPtr int32) int32 {
	return migrateContract(context, codePtr, codeSize, namePtr, nameSize, verPtr, verSize,
		authorPtr, authorSize, emailPtr, emailSize, descPtr, descSize, newAddrPtr)
}

//export destroy_contract
func destroy_contract(context unsafe.Pointer) {
	destroyContract(context)
}

//export panic_contract
func panic_contract(context unsafe.Pointer, dataPtr, dataSize int32) {
	panicContract(context, dataPtr, dataSize)
}

var inputData = map[int32][]byte{}

const (
	InputDataTypeParam          = 0
	InputDataTypeContractResult = 1
)

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
	_, _ = imports.Append("destroy_contract", destroy_contract, C.destroy_contract)
	_, _ = imports.Append("migrate_contract", migrate_contract, C.migrate_contract)
	_, _ = imports.Append("panic_contract", panic_contract, C.panic_contract)

	code := getBytes()
	module, err := wasm.Compile(code)
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

	sendAddr := NewAddress([]byte("user0000000000000000"))
	callAddr := NewAddress([]byte("contract000000000000"))
	params := [][]interface{}{
		{"write_db", "time", "机器"},
		{"read_db", "time"},
		{"delete_db", "time"},
		{"send", sendAddr.ToString(), uint64(7)},
		{"get_creator"},
		{"get_invoker"},
		{"get_time"},
		{"call_contract", callAddr.ToString(), []byte{1, 2, 3}},
		{"destroy_contract"},
		{"migrate_contract", code, "demo", "v0.0.1", "me", "email", "description"},
		{"notify"},
		{"mul", int64(1 << 60), int64(1 << 61), int64(1 << 62), int64(1 << 63 - 1)}, //overflow
		{"这是一个无效的方法"},
		{"send", "a" + sendAddr.ToString()[1:], uint64(7)}, //panic用例
		{"read_db", "不存在的key"}, //rust panic
	}

	for _, param := range params {
		fmt.Printf("\n==============================\ncall %s\n", param[0])
		inputData[InputDataTypeParam] = serialize(param)
		func() {
			defer func() {
				if err := recover(); err != nil {
					fmt.Println(err)
				}
			}()

			_, err = invoke()
			if err != nil {
				panic(err)
			}
		}()
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

		case uint32:
			sink.WriteU32(r)

		case uint64:
			sink.WriteU64(r)

		case int32:
			sink.WriteI32(r)

		case int64:
			sink.WriteI64(r)

		case []byte:
			sink.WriteBytes(r)

		//case Address:
		//	sink.WriteAddress(r)

		default:
			panic("unexpected type")
		}
	}

	return sink.Bytes()
}

func main() {
	ontologyContract()
}
