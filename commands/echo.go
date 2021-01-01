package commands

import (
	"github.com/gookit/color"
	"strings"
)

type Echo struct {

}

func (e Echo) Execute(args []string) error {
	color.Println(strings.Join(args, " "))
	return nil
}


