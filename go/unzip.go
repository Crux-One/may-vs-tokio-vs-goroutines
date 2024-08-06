package main

import (
	"bytes"
	"io"

	"github.com/yeka/zip"
)

func tryUnzip(zipData []byte, password string, result chan<- string) {
	r := bytes.NewReader(zipData)
	zipReader, err := zip.NewReader(r, int64(len(zipData)))
	if err != nil {
		return
	}

	for _, f := range zipReader.File {
		f.SetPassword(password)

		rc, err := f.Open()
		if err != nil {
			continue
		}
		defer rc.Close()

		_, err = io.Copy(io.Discard, rc)
		if err != nil {
			continue
		}

		result <- password
		break
	}
}
