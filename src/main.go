package main

import (
	"fmt"
	"os"
	"strconv"
	"sync"
	"sync/atomic"
	"time"
)

// atomic number to dynamically wait for ALL files to be opened at once
var openCount atomic.Int32

func main() {

	// Get the number of files to bootstrap
	numFiles, err := strconv.Atoi(os.Args[1])
	if err != nil {
		panic(err)
	}

	var wg sync.WaitGroup
	files := make([]*os.File, numFiles)

	// Create 10 temp files
	for i := 0; i < numFiles; i++ {
		tmpfile, err := os.CreateTemp("", fmt.Sprintf("test-%d-*.txt", i))
		if err != nil {
			panic(err)
		}

		// Close the initial handle so we can reopen it
		tmpfile.Close()
		files[i] = tmpfile
	}
	startTime := time.Now()
	// Open all files concurrently using a wait group.
	// Increment wg counter to only 1: the last member of the group
	// will call wg.Done() causing all to finish.
	wg.Add(1)
	for i := 0; i < numFiles; i++ {
		go func(index int) {
			file := files[index]
			f, err := os.OpenFile(file.Name(), os.O_RDWR, 0644)
			if err != nil {
				panic(err)
			}
			files[index] = f

			// Increment the counter and wait if we're not the last one
			// The last one will decrement the sole WG counter
			count := openCount.Add(1)
			if count < int32(numFiles) {
				wg.Wait()
			} else {
				wg.Done()
			}
		}(i)
	}

	// Wait for all files to be opened
	wg.Wait()

	// Cleanup: close and remove all files
	for _, f := range files {
		f.Close()
		os.Remove(f.Name())
	}

	elapsed := time.Since(startTime)
	fmt.Printf("Total execution time: %v\n", elapsed)
}
