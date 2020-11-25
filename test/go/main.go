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
// extern void self_address(void*, int);
// extern void get_pre_caller(void*, int);
// extern void get_block_header(void*, int);
//
// extern int get_input_length(void*, int);
// extern void get_input(void*, int, int, int);
// extern void notify_contract(void*, int, int);
// extern void return_contract(void*, int, int);
// extern int call_contract(void*, int, int, int);
// extern void new_contract(void*, int, int, int, int, int);
// extern void destroy_contract(void*);
// extern void panic_contract(void*, int, int);
// extern void get_validator_power(void*, int, int, int);
// extern void total_power(void*, int);
// extern void get_balance(void*, int, int);
//
// extern void debug_print(void*, int, int);
import "C"
import (
	"encoding/hex"
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

//export self_address
func self_address(context unsafe.Pointer, contractPtr int32) {
	selfAddress(context, contractPtr)
}

//export get_pre_caller
func get_pre_caller(context unsafe.Pointer, callerPtr int32) {
	getPreCaller(context, callerPtr)
}

//export get_block_header
func get_block_header(context unsafe.Pointer, valuePtr int32) {
	getBlockHeader(context, valuePtr)
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

//export new_contract
func new_contract(context unsafe.Pointer, codeHashPtr, codeHashSize, argsPtr, argsSize, newContractPtr int32) {
	newContract(context, codeHashPtr, codeHashSize, argsPtr, argsSize, newContractPtr)
}

//export destroy_contract
func destroy_contract(context unsafe.Pointer) {
	destroyContract(context)
}

//export panic_contract
func panic_contract(context unsafe.Pointer, dataPtr, dataSize int32) {
	panicContract(context, dataPtr, dataSize)
}

//export get_validator_power
func get_validator_power(context unsafe.Pointer, dataPtr, dataSize, valuePtr int32) {
	getValidatorPower(context, dataPtr, dataSize, valuePtr)
}

//export total_power
func total_power(context unsafe.Pointer, valuePtr int32) {
	totalPower(context, valuePtr)
}

//export get_balance
func get_balance(context unsafe.Pointer, addrPtr, balancePtr int32) {
	getBalance(context, addrPtr, balancePtr)
}

//export debug_print
func debug_print(context unsafe.Pointer, msgPtr, msgSize int32) {
	debugPrint(context, msgPtr, msgSize)
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

func shardChainContract() {
	imports, err := wasm.NewImports().Namespace("env").Append("send", send, C.send)
	if err != nil {
		panic(err)
	}

	_, _ = imports.Append("read_db", read_db, C.read_db)
	_, _ = imports.Append("write_db", write_db, C.write_db)
	_, _ = imports.Append("delete_db", delete_db, C.delete_db)

	_, _ = imports.Append("get_creator", get_creator, C.get_creator)
	_, _ = imports.Append("get_invoker", get_invoker, C.get_invoker)
	_, _ = imports.Append("self_address", self_address, C.self_address)
	_, _ = imports.Append("get_pre_caller", get_pre_caller, C.get_pre_caller)
	_, _ = imports.Append("get_block_header", get_block_header, C.get_block_header)

	_, _ = imports.Append("get_input_length", get_input_length, C.get_input_length)
	_, _ = imports.Append("get_input", get_input, C.get_input)
	_, _ = imports.Append("return_contract", return_contract, C.return_contract)
	_, _ = imports.Append("notify_contract", notify_contract, C.notify_contract)
	_, _ = imports.Append("call_contract", call_contract, C.call_contract)
	_, _ = imports.Append("new_contract", new_contract, C.new_contract)
	_, _ = imports.Append("destroy_contract", destroy_contract, C.destroy_contract)
	_, _ = imports.Append("panic_contract", panic_contract, C.panic_contract)

	_, _ = imports.Append("get_validator_power", get_validator_power, C.get_validator_power)
	_, _ = imports.Append("total_power", total_power, C.total_power)
	_, _ = imports.Append("get_balance", get_balance, C.get_balance)

	_, _ = imports.Append("debug_print", debug_print, C.debug_print)

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

	sendAddr := NewAddress([]byte("user0000000000000000"))
	callAddr := NewAddress([]byte("contract000000000000"))
	params := [][]interface{}{
		{"write_db", "time", "机器"},
		{"read_db", "time"},
		{"delete_db", "time"},
		{"send", sendAddr.ToString(), uint64(7)},
		{"get_creator"},
		{"get_invoker"},
		{"self_address"},
		{"get_pre_caller"},
		{"get_block_header"},
		{"call_contract", callAddr.ToString(), []byte{1, 2, 3}},
		{"new_contract", []byte("code hash"), []byte("input")},
		{"destroy_contract"},
		{"notify"},
		{"get_validator_power"},
		{"total_power"},
		{"get_balance", sendAddr.ToString()},
		{"mul", int64(1 << 60), int64(1 << 61), int64(1 << 62), int64(1<<63 - 1)}, //overflow
		{"这是一个无效的方法"},
		{"send", "a" + sendAddr.ToString()[1:], uint64(7)}, //panic用例
		{"read_db", "不存在的key"},                             //rust panic
	}

	for _, param := range params {
		fmt.Printf("\n==============================\ncall %s\n", param[0])
		invoke, exist := instance.Exports["x"+hex.EncodeToString([]byte(param[0].(string)))]
		if !exist {
			fmt.Printf("can not find method: %s\n", param[0])
			continue
		}

		inputData[InputDataTypeParam] = serialize(param[1:])
		func() {
			defer func() {
				if err := recover(); err != nil {
					fmt.Printf("catch: %v\n", err)
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
	shardChainContract()
}
