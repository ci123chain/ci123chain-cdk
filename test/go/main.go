package main

// #include <stdlib.h>
//
// extern int read_db(void *context, int key, int value);
// extern void write_db(void *context, int key, int value);
// extern void delete_db(void *context, int key);
// extern int send(void *context, int toPtr, int amountPtr);
// extern void get_creator(void *context, int creatorPtr);
// extern void get_invoker(void *context, int invokerPtr);
// extern void get_time(void *context, int timePtr);
// extern int get_input_length(void *context);
// extern void get_input(void *context, int ptr);
// extern void return_contract(void *context, int value);
// extern void notify_contract(void *context, int msg);
import "C"
import (
	"encoding/json"
	"fmt"
	wasm "github.com/wasmerio/go-ext-wasm/wasmer"
	"unsafe"
)

var length = 0
var message []byte

//export send
func send(context unsafe.Pointer, toPtr int32, amountPtr int32) int32 {
	return perform_send(context, toPtr, amountPtr)
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
func get_time(context unsafe.Pointer, timePtr int32) {
	getTime(context, timePtr)
}

//export read_db
func read_db(context unsafe.Pointer, key, value int32) int32 {
	return readDB(context, key, value)
}

//export write_db
func write_db(context unsafe.Pointer, key, value int32) {
	writeDB(context, key, value)
}

//export delete_db
func delete_db(context unsafe.Pointer, key int32) {
	deleteDB(context, key)
}

//export get_input_length
func get_input_length(context unsafe.Pointer) int32 {
	return int32(length)
}

//export return_contract
func return_contract(context unsafe.Pointer, value int32) {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	region := NewRegion(memory[value : value+4+4+4])
	fmt.Println(region)
	fmt.Println(string(memory[region.Offset : region.Offset+region.Length]))
}

//export get_input
func get_input(context unsafe.Pointer, ptr int32) {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()
	fmt.Println(ptr, length)
	copy(memory[ptr:ptr+int32(length)], message)
}

//export notify_contract
func notify_contract(context unsafe.Pointer, msg int32) {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	region := NewRegion(memory[msg : msg+4+4+4])
	fmt.Println(region)
	fmt.Println(string(memory[region.Offset : region.Offset+region.Length]))

	type Event struct {
		Type string                 `json:"type"`
		Attr map[string]interface{} `json:"attr"`
	}

	var event Event
	err := json.Unmarshal(memory[region.Offset:region.Offset+region.Length], &event)
	if err != nil {
		panic(err)
	}
	fmt.Println(event)
}

//要在import函数中调用export函数，需要中转函数
type middle struct {
	fun map[string]func(...interface{}) (wasm.Value, error)
}

var middleIns = middle{fun: make(map[string]func(...interface{}) (wasm.Value, error))}

func GetBytes() []byte {
	modulePath := "/media/thomas/_dde_data3/github/ci123chain-cdk/target/wasm32-unknown-unknown/debug/rust_sdk.wasm"

	res, err := wasm.ReadBytes(modulePath)
	if err != nil {
		panic(err)
	}

	return res
}

func erc20Contract() {
	//环境准备
	imports, err := wasm.NewImports().Namespace("env").Append("read_db", read_db, C.read_db)
	if err != nil {
		panic(err)
	}

	_, _ = imports.Namespace("env").Append("write_db", write_db, C.write_db)
	_, _ = imports.Namespace("env").Append("delete_db", delete_db, C.delete_db)
	_, err = imports.Namespace("env").Append("send", send, C.send)
	_, err = imports.Namespace("env").Append("get_creator", get_creator, C.get_creator)
	_, err = imports.Namespace("env").Append("get_invoker", get_invoker, C.get_invoker)
	_, err = imports.Namespace("env").Append("get_time", get_time, C.get_time)
	if err != nil {
		fmt.Println(err.Error())
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

	allocate, exist := instance.Exports["allocate"]
	if !exist {
		fmt.Println(exist)
		return
	}
	middleIns.fun["allocate"] = allocate

	init, exist := instance.Exports["init"]
	if !exist {
		fmt.Println("init not found")
		return
	}

	handle, exist := instance.Exports["handle"]
	if !exist {
		fmt.Println("handle not found")
		return
	}

	query, exist := instance.Exports["query"]
	if !exist {
		fmt.Println("query not found")
		return
	}

	type Param struct {
		Method string   `json:"method"`
		Args   []string `json:"args"`
	}

	//调用
	{
		res, err := wasmCall(instance, init, &Param{
			Method: "init",
			Args:   []string{"token", "addr1", "100000"},
		})
		if err != nil {
			panic(err)
		}

		if res.Err != "" {
			fmt.Println(res.Err)
		} else {
			fmt.Println(string(res.Ok.Data))
		}
	}

	{
		res, err := wasmCall(instance, handle, &Param{
			Method: "transfer",
			Args:   []string{"addr1", "addr2", "500"},
		})
		if err != nil {
			panic(err)
		}
		if res.Err != "" {
			fmt.Println(res.Err)
		} else {
			fmt.Println(string(res.Ok.Data))
		}
	}

	{
		res, err := wasmCall(instance, handle, &Param{
			Method: "transferErr",
			Args:   []string{"addr1", "addr2", "500"},
		})
		if err != nil {
			panic(err)
		}
		if res.Err != "" {
			fmt.Println(res.Err)
		} else {
			fmt.Println(string(res.Ok.Data))
		}
	}

	{
		res, err := wasmCall(instance, query, &Param{
			Method: "balance",
			Args:   []string{"addr1"},
		})
		if err != nil {
			panic(err)
		}

		if res.Err != "" {
			fmt.Println(res.Err)
		} else {
			fmt.Println("addr1 balance :" + string(res.Ok.Data))
		}
	}

	{
		res, err := wasmCall(instance, query, &Param{
			Method: "balance",
			Args:   []string{"addr2"},
		})
		if err != nil {
			panic(err)
		}

		if res.Err != "" {
			fmt.Println(res.Err)
		} else {
			fmt.Println("addr2 balance :" + string(res.Ok.Data))
		}
	}

	{
		res, err := wasmCall(instance, handle, &Param{
			Method: "approve",
			Args:   []string{"addr1", "addr3", "333"},
		})
		if err != nil {
			panic(err)
		}
		if res.Err != "" {
			fmt.Println(res.Err)
		} else {
			fmt.Println(string(res.Ok.Data))
		}
	}

	{
		res, err := wasmCall(instance, query, &Param{
			Method: "allowance",
			Args:   []string{"addr1", "addr3"},
		})
		if err != nil {
			panic(err)
		}
		if res.Err != "" {
			fmt.Println(res.Err)
		} else {
			fmt.Println("addr3 in addr1 allowance :" + string(res.Ok.Data))
		}
	}

	{
		res, err := wasmCall(instance, handle, &Param{
			Method: "transferFrom",
			Args:   []string{"addr1", "addr3", "addr2", "333"},
		})
		if err != nil {
			panic(err)
		}
		if res.Err != "" {
			fmt.Println(res.Err)
		} else {
			fmt.Println(string(res.Ok.Data))
		}
	}

	{
		res, err := wasmCall(instance, query, &Param{
			Method: "balance",
			Args:   []string{"addr2"},
		})
		if err != nil {
			panic(err)
		}
		if res.Err != "" {
			fmt.Println(res.Err)
		} else {
			fmt.Println("addr2 balance :" + string(res.Ok.Data))
		}
	}

	{
		res, err := wasmCall(instance, query, &Param{
			Method: "balance",
			Args:   []string{"addr1"},
		})
		if err != nil {
			panic(err)
		}
		if res.Err != "" {
			fmt.Println(res.Err)
		} else {
			fmt.Println("addr1 balance :" + string(res.Ok.Data))
		}
	}

	{
		res, err := wasmCall(instance, query, &Param{
			Method: "allowance",
			Args:   []string{"addr1", "addr3"},
		})
		if err != nil {
			panic(err)
		}
		if res.Err != "" {
			fmt.Println(res.Err)
		} else {
			fmt.Println("addr3 in addr1 allowance :" + string(res.Ok.Data))
		}
	}

}

type RespW struct {
	Ok  RespN  `json:"ok"`
	Err string `json:"err"`
}

type RespN struct {
	Data []byte `json:"data"`
}

func readCString(memory []byte) *RespW {
	var res []byte
	for i := range memory {
		if memory[i] == 0 {
			break
		}
		res = append(res, memory[i])
	}
	var resp RespW
	err := json.Unmarshal(res, &resp)
	if err != nil {
		fmt.Println(err)
	}
	return &resp
}

func wasmCall(instance wasm.Instance, fun func(...interface{}) (wasm.Value, error), msg interface{}) (*RespW, error) {
	allocate, exist := middleIns.fun["allocate"]
	if !exist {
		panic("allocate not found")
	}

	var data []byte
	switch msg.(type) {
	case string:
		data = []byte(msg.(string))
	default:
		res, err := json.Marshal(msg)
		if err != nil {
			return nil, err
		}
		data = res
	}
	fmt.Println(string(data))
	data = append(data, 0) // c str, + \0

	offset, err := allocate(len(data))
	if err != nil {
		return nil, err
	}
	copy(instance.Memory.Data()[offset.ToI32():offset.ToI32()+int32(len(data))], data)

	res, err := fun(offset)
	if err != nil {
		return nil, err
	}
	return readCString(instance.Memory.Data()[res.ToI32():]), nil
}

func ontologyContract() {
	imports, err := wasm.NewImports().Namespace("env").Append("get_input_length", get_input_length, C.get_input_length)
	if err != nil {
		panic(err)
	}

	_, _ = imports.Append("get_input", get_input, C.get_input)
	_, _ = imports.Append("return_contract", return_contract, C.return_contract)
	_, _ = imports.Append("notify_contract", notify_contract, C.notify_contract)

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

	invoke, exist := instance.Exports["invoke"]
	if !exist {
		fmt.Println(exist)
		return
	}

	type Param struct {
		Method string   `json:"method"`
		Args   []string `json:"args"`
	}

	param := Param{
		Method: "methodRun!!",
		Args:   []string{"zo", "go"},
	}

	msg, err := json.Marshal(param)
	if err != nil {
		panic(err)
	}
	fmt.Println(string(msg))
	length = len(msg)
	message = msg

	_, err = invoke()
	if err != nil {
		panic(err)
	}

}

func main() {
	ontologyContract()
}
