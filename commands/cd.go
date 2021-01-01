package commands

import (
	"os"
)

type Cd struct {

}

func (c Cd) Execute(args []string) error {
	return os.Chdir(args[0])
}



