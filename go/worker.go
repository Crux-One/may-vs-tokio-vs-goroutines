package main

import (
	"sync"
)

func worker(zipData []byte, tasks chan []string, wg *sync.WaitGroup, result chan string) {
	defer wg.Done()

	for passwords := range tasks {
		for _, password := range passwords {
			select {
			case <-result:
				return
			default:
				tryUnzip(zipData, password, result)
			}
		}
	}
}
