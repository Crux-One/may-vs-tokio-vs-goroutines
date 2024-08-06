package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"sync"
	"time"
)

func main() {
	zipPath := "../target.zip"
	dictPath := "../xato-net-10-million-passwords.txt"
	numWorkers := 10

	zipFile, _ := os.Open(zipPath)
	defer zipFile.Close()

	zipData, _ := io.ReadAll(zipFile)

	file, _ := os.Open(dictPath)
	defer file.Close()

	var passwords []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		passwords = append(passwords, scanner.Text())
	}
	if err := scanner.Err(); err != nil {
		fmt.Println("Failed to read dictionary file:", err)
		return
	}

	passwordsPerWorker := len(passwords) / numWorkers

	// fmt.Println("Num of passwords:", len(passwords))
	// fmt.Println("Num of workers:", numWorkers)
	// fmt.Println("Num of passwords / worker:", passwordsPerWorker)

	var wg sync.WaitGroup
	tasks := make(chan []string, numWorkers)
	result := make(chan string, 1)

	start := time.Now()

	for i := 0; i < numWorkers; i++ {
		wg.Add(1)
		go worker(zipData, tasks, &wg, result)
	}

	go func() {
		for i := 0; i < numWorkers; i++ {
			startIdx := i * passwordsPerWorker
			endIdx := startIdx + passwordsPerWorker
			if startIdx >= len(passwords) {
				break
			}
			if endIdx > len(passwords) {
				endIdx = len(passwords)
			}
			tasks <- passwords[startIdx:endIdx]
		}
		close(tasks)
	}()

	go func() {
		wg.Wait()
		close(result)
	}()

	if _, ok := <-result; ok {
		elapsed := time.Since(start).Milliseconds()
		fmt.Printf("%d\n", elapsed)
	} else {
		fmt.Println("Password not found")
	}
}
