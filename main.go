package main

import (
	"fmt"
	"github.com/gookit/color"
	"github.com/jviguy/dev_term/commands"
	"log"
	"os"
	"os/user"
	"strings"
	"time"
)

import "bufio"

func main() {
	fmt.Println(color.FgLightGreen.Render("--- ~~Starting Dev Term~~ ---"))
	cfg := LoadConfig()
	cmdMap := commands.New()
	cmdMap.RegisterCommand("os", commands.Os{})
	cmdMap.RegisterCommand("cd", commands.Cd{})
	cmdMap.RegisterCommand("download", commands.Download{})
	cmdMap.RegisterCommand("echo", commands.Echo{})
	cmdMap.RegisterCommand("ls", commands.Ls{FileColors: cfg.FileColors})
	cmdMap.RegisterCommand("exec", commands.Exec{})
	scnr := bufio.NewScanner(os.Stdin)
	for {
		cuser, _ := user.Current()
		dir, err := os.Getwd()
		if err != nil {
			log.Fatal(err)
		}
		color.Print("<blue>" + cuser.Username +"@" + dir + " $ </>")
		scnr.Scan()
		cmdraw := scnr.Text()
		cmdsplit := strings.Split(cmdraw, " ")
		args := cmdsplit[1:]
		err = cmdMap.ExecuteCommand(cmdsplit[0], args)
		if err != nil {
			color.Println("<error>" + time.Now().Format("January 2, 2006 15:04:05"), err, "</>")
		}
	}
}
