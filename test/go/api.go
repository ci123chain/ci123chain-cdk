package main

import (
	"encoding/hex"
	"encoding/json"
	"fmt"
	"time"
	"unsafe"

	wasm "github.com/wasmerio/go-ext-wasm/wasmer"
)

const AddressSize = 20

type Address [AddressSize]byte

func (addr *Address) ToString() string {
	return hex.EncodeToString(addr[:])
}

func getInputLength(context unsafe.Pointer) int32 {
	return int32(len([]byte(inputData)))
}

func getInput(context unsafe.Pointer, ptr int32, size int32) {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	copy(memory[ptr:ptr+size], inputData)
}

func performSend(context unsafe.Pointer, to int32, amount int64) int32 {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	var toAddr Address
	copy(toAddr[:], memory[to: to + AddressSize])

	fmt.Println("send to: " + toAddr.ToString())
	fmt.Printf("send amount: %d\n", amount)

	//err := accountKeeper.transfer 实际链上校验、转账等
	//if err != nil {
	//	return 1
	//}
	return 0
}

func getCreator(context unsafe.Pointer, CreatorPtr int32) {
	creatorAddr := Address{} //contractAddress
	copy(creatorAddr[:], "addr1111111111111111")

	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	copy(memory[CreatorPtr: CreatorPtr + AddressSize], creatorAddr[:])
}

func getInvoker(context unsafe.Pointer, invokerPtr int32) {
	creatorAddr := Address{}//contractAddress
	copy(creatorAddr[:], "addr2222222222222222")

	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	copy(memory[invokerPtr: invokerPtr + AddressSize], creatorAddr[:])
}

func getTime(context unsafe.Pointer) int64 {
	now := time.Now() //blockHeader.Time
	return now.Unix()
}

func notifyContract(context unsafe.Pointer, ptr, size int32) {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	type Event struct {
		Type string                 `json:"type"`
		Attr map[string]interface{} `json:"attr"`
	}

	var event Event
	err := json.Unmarshal(memory[ptr: ptr + size], &event)
	if err != nil {
		panic(err)
	}
	fmt.Println(event)
}

func returnContract(context unsafe.Pointer, ptr, size int32) {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	result := memory[ptr: ptr + size]

	var resp RespW
	err := json.Unmarshal(result, &resp)
	if err != nil {
		fmt.Println(err)
	}

	fmt.Println(string(resp.Ok.Data))
}

type RespW struct {
	Ok  RespN   `json:"ok"`
	Err string 	`json:"err"`
}

type RespN struct {
	Data []byte `json:"data"`
}