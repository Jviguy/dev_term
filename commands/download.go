package commands

import (
	"github.com/gookit/color"
	"io"
	"net/http"
	"os"
	"path"
)

type Download struct {
	client *http.Client
}

func (d Download) Execute(args []string) error {
	resp, err := http.Get(args[0])
	if err != nil {
		return err
	}
	defer resp.Body.Close()
	color.Println("<green>Downloading to", path.Base(args[0]), "</>")
	f, err := os.Create(path.Base(args[0]))
	if err != nil {
		return err
	}
	defer f.Close()
	_, err = io.Copy(f, resp.Body)
	color.Println("<green>Finished Downloading", path.Base(args[0]), "</>")
	return err
}



