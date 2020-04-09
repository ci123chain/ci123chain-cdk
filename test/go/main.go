package main

// #include <stdlib.h>
//
// extern int read_db(void *context, int key, int value);
// extern void write_db(void *context, int key, int value);
// extern void delete_db(void *context, int key);
import "C"
import (
	"encoding/json"
	"fmt"
	wasm "github.com/wasmerio/go-ext-wasm/wasmer"
	"unsafe"
)

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

//要在import函数中调用export函数，需要中转函数
type middle struct {
	fun map[string]func(...interface{}) (wasm.Value, error)
}

var middleIns = middle{fun: make(map[string]func(...interface{}) (wasm.Value, error))}

func GetBytes() []byte {
	modulePath := "../../target/wasm32-unknown-unknown/debug/rust_sdk.wasm"

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
			Args:   []string{"token","addr1","100000"},
		})
		if err != nil {
			panic(err)
		}
		fmt.Println(string(res.Ok.Data))
	}

	{
		res, err := wasmCall(instance, handle, &Param{
			Method: "transfer",
			Args:   []string{"addr1","addr2","500"},
		})
		if err != nil {
			panic(err)
		}
		fmt.Println(string(res.Ok.Data))
	}

	{
		res, err := wasmCall(instance, query, &Param{
			Method: "balance",
			Args:   []string{"addr1"},
		})
		if err != nil {
			panic(err)
		}
		fmt.Println("addr1 balance :" +string(res.Ok.Data))
	}

	{
		res, err := wasmCall(instance, query, &Param{
			Method: "balance",
			Args:   []string{"addr2"},
		})
		if err != nil {
			panic(err)
		}
		fmt.Println("addr2 balance :" +string(res.Ok.Data))
	}

	{
		res, err := wasmCall(instance, handle, &Param{
			Method: "approve",
			Args:   []string{"addr1","addr3","333"},
		})
		if err != nil {
			panic(err)
		}
		fmt.Println(string(res.Ok.Data))
	}

	{
		res, err := wasmCall(instance, query, &Param{
			Method: "allowance",
			Args:   []string{"addr1","addr3"},
		})
		if err != nil {
			panic(err)
		}
		fmt.Println("addr3 in addr1 allowance :" + string(res.Ok.Data))
	}

	{
		res, err := wasmCall(instance, handle, &Param{
			Method: "transferFrom",
			Args:   []string{"addr1","addr3","addr2","333"},
		})
		if err != nil {
			panic(err)
		}
		fmt.Println(string(res.Ok.Data))
	}

	{
		res, err := wasmCall(instance, query, &Param{
			Method: "balance",
			Args:   []string{"addr2"},
		})
		if err != nil {
			panic(err)
		}
		fmt.Println("addr2 balance :" +string(res.Ok.Data))
	}

	{
		res, err := wasmCall(instance, query, &Param{
			Method: "balance",
			Args:   []string{"addr1"},
		})
		if err != nil {
			panic(err)
		}
		fmt.Println("addr1 balance :" + string(res.Ok.Data))
	}

	{
		res, err := wasmCall(instance, query, &Param{
			Method: "allowance",
			Args:   []string{"addr1","addr3"},
		})
		if err != nil {
			panic(err)
		}
		fmt.Println("addr3 in addr1 allowance :" + string(res.Ok.Data))
	}

}

type RespW struct {
    Ok  RespN   `json:"ok"`
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

func main() {
	erc20Contract()
}
