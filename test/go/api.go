package main

import (
	"encoding/hex"
	"errors"
	"fmt"
	"time"
	"unsafe"

	wasm "github.com/wasmerio/go-ext-wasm/wasmer"
)

const AddressSize = 20

type Address [AddressSize]byte

func NewAddress(raw []byte) (addr Address) {
	if len(raw) != AddressSize {
		panic("mismatch size")
	}

	copy(addr[:], raw)
	return
}

func (addr *Address) ToString() string {
	return "0x" + hex.EncodeToString(addr[:])
}

type Event struct {
	Type string                 `json:"type"`
	Attr map[string]interface{} `json:"attr"`
}

const (
	EventAttrValueTypeInt64  = 0
	EventAttrValueTypeString = 1
)

func NewEventFromSlice(raw []byte) (Event, error) {
	event := Event{
		Attr: map[string]interface{}{},
	}

	sink := NewSink(raw)

	tp, err := sink.ReadString()
	if err != nil {
		return event, err
	}
	event.Type = tp

	sizeOfMap, err := sink.ReadU32()
	if err != nil {
		return event, err
	}

	for i := 0; i < int(sizeOfMap); i++ {
		key, err := sink.ReadString()
		if err != nil {
			return event, err
		}
		typeOfValue, err := sink.ReadByte()
		if err != nil {
			return event, err
		}

		var value interface{}
		switch typeOfValue {
		case EventAttrValueTypeInt64:
			value, err = sink.ReadI64()
		case EventAttrValueTypeString:
			value, err = sink.ReadString()
		default:
			return event, errors.New(fmt.Sprintf("unexpected event attr type: %b", typeOfValue))
		}
		if err != nil {
			return event, err
		}
		event.Attr[key] = value
	}

	return event, nil
}

func getInputLength(_ unsafe.Pointer, token int32) int32 {
	return int32(len(inputData[token]))
}

func getInput(context unsafe.Pointer, token, ptr int32, size int32) {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	copy(memory[ptr:ptr+size], inputData[token])
}

func performSend(context unsafe.Pointer, to int32, amount int64) int32 {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	var toAddr Address
	copy(toAddr[:], memory[to:to+AddressSize])

	fmt.Println("send to: " + toAddr.ToString())
	fmt.Printf("send amount: %d\n", amount)

	//err := accountKeeper.transfer 实际链上校验、转账等
	//if err != nil {
	//	return 1
	//}
	return 1 // 1 代表 bool true
}

func getCreator(context unsafe.Pointer, CreatorPtr int32) {
	creatorAddr := Address{} //contractAddress
	copy(creatorAddr[:], "addr1111111111111111")

	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	copy(memory[CreatorPtr:CreatorPtr+AddressSize], creatorAddr[:])
}

func getInvoker(context unsafe.Pointer, invokerPtr int32) {
	creatorAddr := Address{} //contractAddress
	copy(creatorAddr[:], "addr2222222222222222")

	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	copy(memory[invokerPtr:invokerPtr+AddressSize], creatorAddr[:])
}

func selfAddress(context unsafe.Pointer, contractPtr int32) {
	contractAddress := Address{}
	copy(contractAddress[:], "contract222222222222")

	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	copy(memory[contractPtr:contractPtr+AddressSize], contractAddress[:])
}

func getPreCaller(context unsafe.Pointer, callerPtr int32) {
	contractAddress := Address{}
	copy(contractAddress[:], "caller11222222222222")

	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	copy(memory[callerPtr:callerPtr+AddressSize], contractAddress[:])
}

func getTime(_ unsafe.Pointer) int64 {
	now := time.Now() //blockHeader.Time
	return now.Unix()
}

func notifyContract(context unsafe.Pointer, ptr, size int32) {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	event, err := NewEventFromSlice(memory[ptr : ptr+size])
	if err != nil {
		fmt.Println(err)
	}
	fmt.Println(event)
}

func returnContract(context unsafe.Pointer, ptr, size int32) {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	result := memory[ptr : ptr+size]

	sink := NewSink(result)
	ok, err := sink.ReadBool()
	if err != nil {
		fmt.Println(err)
		return
	}
	msg, _, err := sink.ReadBytes()
	if err != nil {
		fmt.Println(err)
		return
	}
	if ok {
		fmt.Printf("ok msg: %s\n", string(msg))
	} else {
		fmt.Printf("error msg: %s\n", string(msg))
	}
}

func callContract(context unsafe.Pointer, addrPtr, inputPtr, inputSize int32) int32 {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	var addr Address
	copy(addr[:], memory[addrPtr:addrPtr+AddressSize])

	input := memory[inputPtr : inputPtr+inputSize]

	fmt.Println("call contract: " + addr.ToString())
	fmt.Print("call param: ")
	fmt.Println(input)

	token := int32(InputDataTypeContractResult)
	inputData[token] = []byte("return value from called contract")

	return token
}

func destroyContract(_ unsafe.Pointer) {
	fmt.Println("destroy contract")
}

func panicContract(context unsafe.Pointer, dataPtr, dataSize int32) {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	data := memory[dataPtr : dataPtr+dataSize]
	panic("contract panic: " + string(data))
}

func getValidatorPower(context unsafe.Pointer, dataPtr, dataSize, valuePtr int32) {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	source := NewSink(memory[dataPtr : dataPtr+dataSize])

	var validators []Address
	{
		length, err := source.ReadU32()
		if err != nil {
			panic(err)
		}
		validators = make([]Address, 0, length)
		var i uint32 = 0
		for ; i < length; i++ {
			bytes, _, err := source.ReadBytes()
			if err != nil {
				panic(err)
			}
			validators = append(validators, NewAddress(bytes))
		}
	}

	//根据链上信息返回验证者的 delegate shares
	value := make([]uint64, len(validators))
	for i := range value {
		value[i] = uint64(i)
	}

	sink := NewSink([]byte{})
	for i := range value {
		sink.WriteU64(value[i])
	}

	res := sink.Bytes()
	copy(memory[valuePtr:int(valuePtr)+len(res)], res)
}

func totalPower(_ unsafe.Pointer) int64 {
	//根据链上信息返回总权益
	return 123456789
}

func debugPrint(context unsafe.Pointer, msgPtr, msgSize int32) {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	data := memory[msgPtr : msgPtr+msgSize]
	println(string(data))
}
