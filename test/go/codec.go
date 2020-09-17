package main

import (
	"bytes"
	"encoding/binary"
	"errors"
	"math/big"
	"unicode/utf8"
)

type RustU128 [16]byte

func NewRustU128(i *big.Int) *RustU128 {
	ib := i.Bytes() // big-endian

	size := len(ib)
	if size > 16 {
		panic("u128最大16字节") //链上处理
	}

	// little-endian
	for i := 0; i < size/2; i++ {
		ib[i], ib[size-1-i] = ib[size-1-i], ib[i]
	}

	// 补全
	for i := 0; i+size < 16; i++ {
		ib = append(ib, 0)
	}

	var u RustU128
	copy(u[:], ib)
	return &u
}

func (u128 *RustU128) Bytes() []byte {
	return u128[:]
}

type Sink struct {
	buf *bytes.Buffer
}

func NewSink(raw []byte) Sink {
	return Sink{
		bytes.NewBuffer(raw),
	}
}

func (sink Sink) WriteU32(i uint32) {
	sink.writeLittleEndian(i)
}

func (sink Sink) WriteU64(i uint64) {
	sink.writeLittleEndian(i)
}

func (sink Sink) WriteU128(u *RustU128) {
	sink.writeRawBytes(u[:])
}

func (sink Sink) WriteI32(i int32) {
	sink.writeLittleEndian(i)
}

func (sink Sink) WriteI64(i int64) {
	sink.writeLittleEndian(i)
}

func (sink Sink) WriteString(s string) {
	sink.WriteU32(uint32(len(s)))
	sink.buf.WriteString(s)
}

func (sink Sink) WriteBytes(b []byte) {
	sink.WriteU32(uint32(len(b)))
	sink.writeRawBytes(b)
}

func (sink Sink) WriteAddress(addr Address) {
	sink.writeRawBytes(addr[:])
}

func (sink Sink) writeRawBytes(b []byte) {
	sink.buf.Write(b)
}

func (sink Sink) writeLittleEndian(i interface{}) {
	_ = binary.Write(sink.buf, binary.LittleEndian, i)
}

func (sink Sink) Bytes() []byte {
	return sink.buf.Bytes()
}

func (sink Sink) ReadBool() (bool, error) {
	b, err := sink.buf.ReadByte()
	if err != nil {
		return false, err
	}
	if b == 0 {
		return false, nil
	}
	return true, nil
}

func (sink Sink) ReadByte() (byte, error) {
	return sink.buf.ReadByte()
}

func (sink Sink) ReadU32() (result uint32, err error) {
	err = binary.Read(sink.buf, binary.LittleEndian, &result)
	return
}

func (sink Sink) ReadI64() (result int64, err error) {
	err = binary.Read(sink.buf, binary.LittleEndian, &result)
	return
}

func (sink Sink) ReadBytes() ([]byte, int, error) {
	size, err := sink.ReadU32()
	if err != nil {
		return nil, 0, err
	}
	return sink.nextBytes(int(size))
}

func (sink Sink) ReadString() (string, error) {
	b, _, err := sink.ReadBytes()
	if err == nil && !utf8.Valid(b) {
		return "", errors.New("invalid utf8 string")
	}

	return string(b), err
}

func (sink Sink) nextBytes(size int) ([]byte, int, error) {
	result := make([]byte, size)
	n, err := sink.buf.Read(result)
	return result, n, err
}
