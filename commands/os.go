package commands

import (
	"fmt"
	"runtime"
)

type Os struct {

}

func (o Os) Execute(_ []string) error {
	_, err := fmt.Println("You are running", runtime.GOOS, "with the", runtime.GOARCH, "architecture!")
	return err
}
