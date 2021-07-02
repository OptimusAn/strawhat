package strawhat

import (
	"os"
	"testing"

	"github.com/golang/protobuf/proto"
)

func TestMarshalAndUnmarshal(t *testing.T) {
	testMarshalAndUnmarshal(t)
}

func TestMarshalToFile(t *testing.T) {
	//testMarshalToFile(t)
}

func TestUnmarshalFromFile(t *testing.T) {
	//testUnmarshalFromFile(t)
}

func testMarshalAndUnmarshal(t *testing.T) {
	strawhat := &Strawhat{
		ServiceID: "yyytest123",
		Protocol:  Protocol_Tcp,
	}
	data, err := proto.Marshal(strawhat)
	if err != nil {
		t.Error(err)
	}

	println("length:", len(data))

	newStrawhat := &Strawhat{}
	if err := proto.Unmarshal(data, newStrawhat); err != nil {
		t.Error(err)
	}

	println("println:", newStrawhat.String())
}

func testMarshalToFile(t *testing.T) {
	strawhat := &Strawhat{
		ServiceID: "yyytest123",
		Protocol:  Protocol_Tcp,
	}
	file, err := os.Create(".test")
	if err != nil {
		t.Error(err)
	}
	data, err := proto.Marshal(strawhat)
	if err != nil {
		t.Error(err)
	}
	if _, err := file.Write(data); err != nil {
		t.Error(err)
	}
}

func testUnmarshalFromFile(t *testing.T) {
	file, err := os.Open(".test")
	if err != nil {
		t.Error(err)
	}
	payload := [1024]byte{}
	n, err := file.Read(payload[:])
	if err != nil {
		t.Error(err)
	}
	newStrawhat := &Strawhat{}
	if err := proto.Unmarshal(payload[:n], newStrawhat); err != nil {
		t.Error(err)
	}
	println("println:", newStrawhat.String())
}

