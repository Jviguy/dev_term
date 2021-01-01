package main

import (
	"encoding/json"
	"io/ioutil"
)

type Config struct {
	FileColors map[string]string
}

func LoadConfig() Config {
	c := Config{}
	data, _ := ioutil.ReadFile("./configuration/file_colors.json")
	_ = json.Unmarshal(data, &c.FileColors)
	if len(c.FileColors) <= 0 {
		c.FileColors = make(map[string]string)
		c.FileColors["go"] = "green"
	}
	data, _ = json.MarshalIndent(c.FileColors, "", " ")
	_ = ioutil.WriteFile("./configuration/file_colors.json", data, 0644)
	return c
}
