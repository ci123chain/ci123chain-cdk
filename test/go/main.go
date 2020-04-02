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

func dbTest() {
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

	{
		type InitMsg struct {
			Count int32 `json:"count"`
		}
		type HandleMsgInc struct{}
		type HandleMsgReset struct {
		}
		msg := InitMsg{Count: 3}
		data, err := json.Marshal(&msg)
		if err != nil {
			panic(err)
		}
		offset, err := allocate(len(data))
		if err != nil {
			panic(err)
		}
		copy(instance.Memory.Data()[offset.ToI32():offset.ToI32()+int32(len(data))], data)

		res, err := init(offset)
		if err != nil {
			panic(err)
		}
		var bytes []byte
		data = instance.Memory.Data()[res.ToI32():]
		for i := range data {
			if data[i] == 0 {
				break
			}
			bytes = append(bytes, data[i])
		}
		fmt.Println(string(bytes))

		//write key [testcount], value [3]
		//{"ok":{"messages":[],"log":[],"data":[123,34,99,111,117,110,116,34,58,51,125]}}
	}
}

func main() {
	dbTest()
}
