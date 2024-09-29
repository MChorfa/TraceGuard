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

	// initialize Dagger client
	client, err := dagger.Connect(ctx, dagger.WithLogOutput(os.Stdout))
	if err != nil {
		return err
	}
	defer client.Close()

	// get reference to the local project
	src := client.Host().Directory(".")

	// create a cache volume
	cache := client.CacheVolume("cargo-cache")

	// define build steps
	rust := client.Container().
		From("rust:latest").
		WithMountedCache("/usr/local/cargo/registry", cache).
		WithDirectory("/app", src).
		WithWorkdir("/app").
		WithExec([]string{"cargo", "build", "--release"})

	// execute
	_, err = rust.Stdout(ctx)

	return err
}
