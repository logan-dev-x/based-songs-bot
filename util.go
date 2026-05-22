package main

import "log"

func check(e error) {
	if e != nil {
		log.Println(e)
	}
}

func checkFatal(e error) {
	if e != nil {
		log.Fatal(e)
	}
}
