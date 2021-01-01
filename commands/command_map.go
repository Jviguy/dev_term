package commands

import (
	"fmt"
	"log"
)

type CommandMap struct {
	cmds map[string]Command
}

func (cmdMap *CommandMap) CommandExist(name string) bool {
	_, ok := cmdMap.cmds[name]
	return ok
}

func (cmdMap *CommandMap) GetCommand(name string) Command {
	return cmdMap.cmds[name]
}

func (cmdMap *CommandMap) ExecuteCommand(name string, args []string) error {
	if !cmdMap.CommandExist(name) {
		return fmt.Errorf("Unknown Command: %s", name)
	}
	cmd := cmdMap.GetCommand(name)
	err := cmd.Execute(args)
	if err != nil {
		log.Println("Error while executing that command: ", err)
	}
	return nil
}

func (cmdMap *CommandMap) RegisterCommand(name string, command Command)  {
	cmdMap.cmds[name] = command
}

func New() *CommandMap {
	return &CommandMap{cmds: make(map[string]Command)}
}