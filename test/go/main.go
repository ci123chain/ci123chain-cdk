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

func countContract() {
	//环境准备
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
		fmt.Println("init not found")
		return
	}

	handle, exist := instance.Exports["handle"]
	if !exist {
		fmt.Println("handle not found")
		return
	}

	//query, exist := instance.Exports["query"]
	//if !exist {
	//	fmt.Println("query not found")
	//	return
	//}

	//调用
	{
		// rust 类型比较复杂, json 参考 https://serde.rs/json.html
		type InitMsg struct {
			Count int32 `json:"count"`
		}
		type ResetMsg struct {
			Count int32 `json:"count"`
		}
		type HandleMsgReset struct {
			Reset ResetMsg `json:"reset"`
		}
		incrementMsg := "increment"
		//queryMsg := "null"

		{
			res, err := wasmCall(instance, init, &InitMsg{Count: 3})
			if err != nil {
				panic(err)
			}
			fmt.Println(res)
		}

		{
			res, err := wasmCall(instance, handle, incrementMsg)
			if err != nil {
				panic(err)
			}
			fmt.Println(res)
		}

		{
			res, err := wasmCall(instance, handle, &HandleMsgReset{Reset: ResetMsg{Count: 10}})
			if err != nil {
				panic(err)
			}
			fmt.Println(res)
		}

		//{
		//	res, err := wasmCall(instance, query, queryMsg)
		//	if err != nil {
		//		panic(err)
		//	}
		//	fmt.Println(res)
		//}

		//write key [test-count], value [3]
		//{"ok":{"messages":[],"log":[],"data":[123,34,99,111,117,110,116,34,58,51,125]}}
	}
}

func readCString(memory []byte) string {
	var res []byte
	for i := range memory {
		if memory[i] == 0 {
			break
		}
		res = append(res, memory[i])
	}
	return string(res)
}

func wasmCall(instance wasm.Instance, fun func(...interface{}) (wasm.Value, error), msg interface{}) (string, error) {
	allocate, exist := middleIns.fun["allocate"]
	if !exist {
		panic("allocate not found")
	}

	data, err := json.Marshal(msg)
	if err != nil {
		return "", err
	}
	fmt.Println(string(data))
	offset, err := allocate(len(data))
	if err != nil {
		return "", err
	}
	copy(instance.Memory.Data()[offset.ToI32():offset.ToI32()+int32(len(data))], data)

	res, err := fun(offset)
	if err != nil {
		return "", err
	}
	return readCString(instance.Memory.Data()[res.ToI32():]), nil
}

func main() {
	countContract()
}
