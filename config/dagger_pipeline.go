package main

import (
	"context"
	"fmt"
	"os"

	"dagger.io/dagger"
)

func main() {
	if err := build(context.Background()); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

func build(ctx context.Context) error {
	fmt.Println("Building with Dagger")

	client, err := dagger.Connect(ctx, dagger.WithLogOutput(os.Stdout))
	if err != nil {
		return err
	}
	defer client.Close()

	src := client.Host().Directory(".")

	rust := client.Container().
		From("rust:latest").
		WithDirectory("/app", src).
		WithWorkdir("/app").
		WithExec([]string{"cargo", "build", "--release"})

	_, err = rust.ExitCode(ctx)
	if err != nil {
		return err
	}

	fmt.Println("Build completed successfully")
	return nil
}
