package main

import (
	"fmt"
	"log"
	"net/http"
)

func main() {
	fmt.Printf("Starting server at port 9443\n")
	if err := http.ListenAndServe(":9443", nil); err != nil {
		log.Fatal(err)
	}

}
