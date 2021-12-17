package main

import (
	"flag"
	"fmt"
	"net/http"
	"os"
)

var addr = flag.String("a", ":8080", "address")

func main() {
	fmt.Printf("pid::::: %d\n", os.Getpid())
	flag.Parse()
	fmt.Println("run")
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
	    fmt.Println("111111")
		fmt.Fprintf(w, "hello")
	})
	if err := http.ListenAndServe(*addr, nil); err != nil {
	    panic(err)
	}
}
