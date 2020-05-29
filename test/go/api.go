package main

import (
	"fmt"
	"time"
	"unsafe"

	wasm "github.com/wasmerio/go-ext-wasm/wasmer"
)

func get_input_length(context unsafe.Pointer) int32 {
	return len([]byte(inputData))
}

func get_input(context unsafe.Pointer, dataPtr, length int32) {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()
	copy(memory[dataPtr.ToI32():dataPtr.ToI32()+int32(length)], []byte(inputData))
}

func perform_send(context unsafe.Pointer, toPtr int32, amountPtr int32) int32 {
	var err error

	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	toAddr := NewRegion(memory[toPtr : toPtr+RegionSize])
	to := memory[toAddr.Offset : toAddr.Offset+toAddr.Length]

	amountAddr := NewRegion(memory[amountPtr : amountPtr+RegionSize])
	amount := memory[amountAddr.Offset : amountAddr.Offset+amountAddr.Length]

	fmt.Println("go get:" + string(to))
	fmt.Println("go get:" + string(amount))

	// err := accountKeeper.transfer 实际链上校验、转账等
	if err != nil {
		return 1
	}
	return 0
}

func getCreator(context unsafe.Pointer, CreatorPtr int32) {
	creatorStr := "addr1" //contractAddress
	size := len(creatorStr)

	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()
	allocate, exist := middleIns.fun["allocate"]
	if !exist {
		panic("allocate not found")
	}
	valueOffset, err := allocate(size)
	if err != nil {
		panic(err)
	}
	copy(memory[valueOffset.ToI32():valueOffset.ToI32()+int32(size)], creatorStr)
	region := Region{
		Offset:   uint32(valueOffset.ToI32()),
		Capacity: uint32(size),
		Length:   uint32(size),
	}
	copy(memory[CreatorPtr:CreatorPtr+RegionSize], region.ToBytes())
}

func getInvoker(context unsafe.Pointer, invokerPtr int32) {
	invokerStr := "addr2" //invokerAddress
	size := len(invokerStr)

	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()
	allocate, exist := middleIns.fun["allocate"]
	if !exist {
		panic("allocate not found")
	}
	valueOffset, err := allocate(size)
	if err != nil {
		panic(err)
	}
	copy(memory[valueOffset.ToI32():valueOffset.ToI32()+int32(size)], invokerStr)
	region := Region{
		Offset:   uint32(valueOffset.ToI32()),
		Capacity: uint32(size),
		Length:   uint32(size),
	}
	copy(memory[invokerPtr:invokerPtr+RegionSize], region.ToBytes())
}

func getTime(context unsafe.Pointer, timePtr int32) {
	tNow := time.Now() //blockHeader.Time
	tStr := tNow.Format("2006-01-02 15:04:05")
	size := len(tStr)

	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()
	allocate, exist := middleIns.fun["allocate"]
	if !exist {
		panic("allocate not found")
	}
	valueOffset, err := allocate(size)
	if err != nil {
		panic(err)
	}
	copy(memory[valueOffset.ToI32():valueOffset.ToI32()+int32(size)], tStr)
	region := Region{
		Offset:   uint32(valueOffset.ToI32()),
		Capacity: uint32(size),
		Length:   uint32(size),
	}
	copy(memory[timePtr:timePtr+RegionSize], region.ToBytes())
}
