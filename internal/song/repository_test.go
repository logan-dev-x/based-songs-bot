package song

import (
	"database/sql"
	"testing"
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Cant open memory DB: %v", err)
	}
	migration := `
	CREATE TABLE IF NOT EXISTS songs (
		fileId TEXT NOT NULL,
		day INTEGER NOT NULL,
		month INTEGER NOT NULL
	);`
	if _, err := db.Exec(migration); err != nil {
		t.Fatalf("Error creating table: %v", err)
	}
	return db
}
