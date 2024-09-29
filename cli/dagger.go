package main

import (
	"context"
	"fmt"

	"dagger.io/dagger"
)

func runDaggerPipeline(ctx context.Context) error {
	client, err := dagger.Connect(ctx)
	if err != nil {
		return err
	}
	defer client.Close()

	src := client.Host().Directory(".")

	golang := client.Container().
		From("golang:1.20").
		WithDirectory("/src", src).
		WithWorkdir("/src")

	runner := golang.WithExec([]string{"go", "build", "-o", "traceguard", "."})

	_, err = runner.Stdout(ctx)
	if err != nil {
		return err
	}

	fmt.Println("Build successful")
	return nil
}
