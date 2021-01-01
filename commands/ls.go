package commands

import (
	"fmt"
	"github.com/gookit/color"
	"os"
	"path/filepath"
	"strings"
)

type Ls struct {
	FileColors map[string]string
}

func (l Ls) Execute(args []string) error {
	if len(args) > 0 && args[0] == "-p" {
		err := filepath.Walk(".",
			func(path string, info os.FileInfo, err error) error {
				if err != nil {
					return err
				}
				if col, ok := l.FileColors[strings.TrimPrefix(filepath.Ext(path), ".")]; ok {
					color.Println("<" + col + ">" + path + "</>")
				} else {
					fmt.Println(path)
				}
				return nil
			})
		if err != nil {
			return err
		}
		return nil
	}
	dir, err := os.Getwd()
	if err != nil {
		return err
	}
	err = filepath.Walk(dir,
		func(path string, info os.FileInfo, err error) error {
			if err != nil {
				return err
			}
			fmt.Println(path)
			return nil
		})
	if err != nil {
		return err
	}
	return nil
}



