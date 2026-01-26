// CIRISRegistry server entrypoint.
//
// Connects to PostgreSQL, runs migrations, and starts gRPC services.
package main

import (
	"context"
	"flag"
	"fmt"
	"log/slog"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/cirisai/cirisregistry/internal/database"
)

func main() {
	// Parse flags
	var (
		dbHost     = flag.String("db-host", envOrDefault("DB_HOST", "localhost"), "Database host")
		dbPort     = flag.Int("db-port", envOrDefaultInt("DB_PORT", 5432), "Database port")
		dbUser     = flag.String("db-user", envOrDefault("DB_USER", "ciris"), "Database user")
		dbPassword = flag.String("db-password", envOrDefault("DB_PASSWORD", ""), "Database password")
		dbName     = flag.String("db-name", envOrDefault("DB_NAME", "ciris_registry"), "Database name")
		dbSSLMode  = flag.String("db-sslmode", envOrDefault("DB_SSLMODE", "require"), "Database SSL mode")
		grpcPort   = flag.Int("grpc-port", envOrDefaultInt("GRPC_PORT", 50051), "gRPC server port")
		logLevel   = flag.String("log-level", envOrDefault("LOG_LEVEL", "info"), "Log level (debug, info, warn, error)")
	)
	flag.Parse()

	// Setup logger
	level := parseLogLevel(*logLevel)
	logger := slog.New(slog.NewJSONHandler(os.Stdout, &slog.HandlerOptions{Level: level}))
	slog.SetDefault(logger)

	logger.Info("starting CIRISRegistry",
		"version", "0.1.0",
		"grpc_port", *grpcPort,
	)

	// Context with cancellation
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Database configuration
	dbConfig := database.Config{
		Host:            *dbHost,
		Port:            *dbPort,
		User:            *dbUser,
		Password:        *dbPassword,
		Database:        *dbName,
		SSLMode:         *dbSSLMode,
		MaxOpenConns:    25,
		MaxIdleConns:    5,
		ConnMaxLifetime: 5 * time.Minute,
		ConnMaxIdleTime: 1 * time.Minute,
	}

	// Connect and run migrations
	db, err := database.ConnectAndMigrate(ctx, dbConfig, logger)
	if err != nil {
		logger.Error("database initialization failed", "error", err)
		os.Exit(1)
	}
	defer db.Close()

	// TODO: Initialize gRPC server
	// server := grpc.NewServer(...)
	// pb.RegisterRegistryServiceServer(server, ...)
	// pb.RegisterPortalServiceServer(server, ...)

	logger.Info("CIRISRegistry ready",
		"grpc_port", *grpcPort,
	)

	// Wait for shutdown signal
	sigCh := make(chan os.Signal, 1)
	signal.Notify(sigCh, syscall.SIGINT, syscall.SIGTERM)
	<-sigCh

	logger.Info("shutting down")

	// TODO: Graceful shutdown
	// server.GracefulStop()
}

func envOrDefault(key, defaultValue string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return defaultValue
}

func envOrDefaultInt(key string, defaultValue int) int {
	if v := os.Getenv(key); v != "" {
		var i int
		if _, err := fmt.Sscanf(v, "%d", &i); err == nil {
			return i
		}
	}
	return defaultValue
}

func parseLogLevel(s string) slog.Level {
	switch s {
	case "debug":
		return slog.LevelDebug
	case "info":
		return slog.LevelInfo
	case "warn":
		return slog.LevelWarn
	case "error":
		return slog.LevelError
	default:
		return slog.LevelInfo
	}
}
