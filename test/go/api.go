package main

import (
	"encoding/hex"
	"errors"
	"fmt"
	"time"
	"unicode/utf8"
	"unsafe"

	wasm "github.com/wasmerio/go-ext-wasm/wasmer"
)

type Param struct {
	Method string   `json:"method"`
	Args   []string `json:"args"`
}

func NewParamFromSlice(raw []byte) (Param, error) {
	var param Param

	sink := NewSink(raw)
	method, err := sink.ReadString()
	if err != nil {
		return param, err
	}
	param.Method = method

	size, err := sink.ReadU32()
	if err != nil {
		return param, err
	}

	for i := 0; i < int(size); i++ {
		arg, err := sink.ReadString()
		if err != nil {
			return param, err
		}
		param.Args = append(param.Args, arg)
	}

	return param, nil
}

func (param Param) Serialize() []byte {
	// 参数必须是合法的UTF8字符串
	if !utf8.ValidString(param.Method) {
		panic("invalid string")
	}
	for i := range param.Args {
		if !utf8.ValidString(param.Args[i]) {
			panic("invalid string")
		}
	}

	sink := NewSink([]byte{})
	sink.WriteString(param.Method)
	sink.WriteU32(uint32(len(param.Args)))
	for i := range param.Args {
		sink.WriteString(param.Args[i])
	}

	return sink.Bytes()
}

const AddressSize = 20

type Address [AddressSize]byte

func (addr *Address) ToString() string {
	return hex.EncodeToString(addr[:])
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
	copy(toAddr[:], memory[to:to+AddressSize])

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

	copy(memory[CreatorPtr:CreatorPtr+AddressSize], creatorAddr[:])
}

func getInvoker(context unsafe.Pointer, invokerPtr int32) {
	creatorAddr := Address{} //contractAddress
	copy(creatorAddr[:], "addr2222222222222222")

	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	copy(memory[invokerPtr:invokerPtr+AddressSize], creatorAddr[:])
}

func getTime(context unsafe.Pointer) int64 {
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
	length, err := sink.ReadU32()
	if err != nil {
		fmt.Println(err)
		return
	}
	msg, _, err := sink.ReadBytes(int(length))
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

func callContract(context unsafe.Pointer, addrPtr, paramPtr, paramSize int32) int32 {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	var addr Address
	copy(addr[:], memory[addrPtr: addrPtr + AddressSize])

	param, err := NewParamFromSlice(memory[paramPtr: paramPtr+ paramSize])
	if err != nil {
		fmt.Println(err)
	}

	fmt.Println("call contract: " + addr.ToString())
	fmt.Print("call param: ")
	fmt.Println(param)

	return 1
}
