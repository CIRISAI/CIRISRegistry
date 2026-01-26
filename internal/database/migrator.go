// Package database provides database connection and migration utilities.
package database

import (
	"context"
	"crypto/sha256"
	"database/sql"
	"embed"
	"encoding/hex"
	"fmt"
	"io/fs"
	"log/slog"
	"path/filepath"
	"sort"
	"strings"
	"time"

	_ "github.com/lib/pq" // PostgreSQL driver
)

//go:embed migrations/*.sql
var migrationsFS embed.FS

// Migration represents a database migration.
type Migration struct {
	Version     string
	Description string
	SQL         string
	Checksum    string
}

// Migrator handles database migrations.
type Migrator struct {
	db     *sql.DB
	logger *slog.Logger
}

// NewMigrator creates a new migrator instance.
func NewMigrator(db *sql.DB, logger *slog.Logger) *Migrator {
	if logger == nil {
		logger = slog.Default()
	}
	return &Migrator{db: db, logger: logger}
}

// loadMigrations reads all migration files from embedded filesystem.
func (m *Migrator) loadMigrations() ([]Migration, error) {
	var migrations []Migration

	entries, err := fs.ReadDir(migrationsFS, "migrations")
	if err != nil {
		return nil, fmt.Errorf("reading migrations directory: %w", err)
	}

	for _, entry := range entries {
		if entry.IsDir() || !strings.HasSuffix(entry.Name(), ".sql") {
			continue
		}

		content, err := fs.ReadFile(migrationsFS, filepath.Join("migrations", entry.Name()))
		if err != nil {
			return nil, fmt.Errorf("reading migration %s: %w", entry.Name(), err)
		}

		// Parse version from filename (e.g., "001_enums_and_extensions.sql")
		parts := strings.SplitN(entry.Name(), "_", 2)
		if len(parts) < 2 {
			return nil, fmt.Errorf("invalid migration filename: %s", entry.Name())
		}

		version := parts[0]
		description := strings.TrimSuffix(parts[1], ".sql")

		// Calculate checksum
		hash := sha256.Sum256(content)
		checksum := hex.EncodeToString(hash[:])

		migrations = append(migrations, Migration{
			Version:     version,
			Description: description,
			SQL:         string(content),
			Checksum:    checksum,
		})
	}

	// Sort by version
	sort.Slice(migrations, func(i, j int) bool {
		return migrations[i].Version < migrations[j].Version
	})

	return migrations, nil
}

// ensureMigrationsTable creates the schema_migrations table if it doesn't exist.
func (m *Migrator) ensureMigrationsTable(ctx context.Context) error {
	_, err := m.db.ExecContext(ctx, `
		CREATE TABLE IF NOT EXISTS schema_migrations (
			version         TEXT PRIMARY KEY,
			description     TEXT,
			applied_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
			checksum        TEXT
		)
	`)
	return err
}

// getAppliedMigrations returns a map of applied migration versions to checksums.
func (m *Migrator) getAppliedMigrations(ctx context.Context) (map[string]string, error) {
	rows, err := m.db.QueryContext(ctx, "SELECT version, checksum FROM schema_migrations")
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	applied := make(map[string]string)
	for rows.Next() {
		var version, checksum string
		if err := rows.Scan(&version, &checksum); err != nil {
			return nil, err
		}
		applied[version] = checksum
	}
	return applied, rows.Err()
}

// Run executes all pending migrations.
func (m *Migrator) Run(ctx context.Context) error {
	m.logger.Info("starting database migration")

	if err := m.ensureMigrationsTable(ctx); err != nil {
		return fmt.Errorf("creating migrations table: %w", err)
	}

	migrations, err := m.loadMigrations()
	if err != nil {
		return fmt.Errorf("loading migrations: %w", err)
	}

	applied, err := m.getAppliedMigrations(ctx)
	if err != nil {
		return fmt.Errorf("getting applied migrations: %w", err)
	}

	for _, migration := range migrations {
		if existingChecksum, ok := applied[migration.Version]; ok {
			// Already applied - verify checksum
			if existingChecksum != migration.Checksum {
				return fmt.Errorf(
					"migration %s checksum mismatch: expected %s, got %s (migration file was modified after being applied)",
					migration.Version, existingChecksum, migration.Checksum,
				)
			}
			m.logger.Debug("migration already applied", "version", migration.Version)
			continue
		}

		// Apply migration
		m.logger.Info("applying migration",
			"version", migration.Version,
			"description", migration.Description,
		)

		start := time.Now()
		if err := m.applyMigration(ctx, migration); err != nil {
			return fmt.Errorf("applying migration %s: %w", migration.Version, err)
		}

		m.logger.Info("migration applied",
			"version", migration.Version,
			"duration", time.Since(start),
		)
	}

	m.logger.Info("database migration complete")
	return nil
}

// applyMigration runs a single migration in a transaction.
func (m *Migrator) applyMigration(ctx context.Context, migration Migration) error {
	tx, err := m.db.BeginTx(ctx, nil)
	if err != nil {
		return fmt.Errorf("beginning transaction: %w", err)
	}
	defer tx.Rollback()

	// Execute migration SQL
	if _, err := tx.ExecContext(ctx, migration.SQL); err != nil {
		return fmt.Errorf("executing SQL: %w", err)
	}

	// Record migration
	_, err = tx.ExecContext(ctx, `
		INSERT INTO schema_migrations (version, description, checksum)
		VALUES ($1, $2, $3)
	`, migration.Version, migration.Description, migration.Checksum)
	if err != nil {
		return fmt.Errorf("recording migration: %w", err)
	}

	return tx.Commit()
}

// Status returns the current migration status.
func (m *Migrator) Status(ctx context.Context) ([]MigrationStatus, error) {
	if err := m.ensureMigrationsTable(ctx); err != nil {
		return nil, err
	}

	migrations, err := m.loadMigrations()
	if err != nil {
		return nil, err
	}

	applied, err := m.getAppliedMigrations(ctx)
	if err != nil {
		return nil, err
	}

	var status []MigrationStatus
	for _, migration := range migrations {
		s := MigrationStatus{
			Version:     migration.Version,
			Description: migration.Description,
			Checksum:    migration.Checksum,
		}
		if checksum, ok := applied[migration.Version]; ok {
			s.Applied = true
			s.ChecksumMatch = checksum == migration.Checksum
		}
		status = append(status, s)
	}

	return status, nil
}

// MigrationStatus represents the status of a single migration.
type MigrationStatus struct {
	Version       string
	Description   string
	Checksum      string
	Applied       bool
	ChecksumMatch bool
}
