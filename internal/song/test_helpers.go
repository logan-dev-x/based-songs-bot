package song

import (
	"database/sql"
	"testing"
)

func setupTestDB(t *testing.T) *sql.DB {
	t.Helper()
	db, err := sql.Open("sqlite3", "file::memory:")
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

func insertMock(t *testing.T, db *sql.DB, fileID string, day int, month int) {
	t.Helper()
	db.Exec("INSERT INTO songs (fileID, day, month) VALUES (?,?,?);", fileID, day, month)
}

func newTestRepository(t *testing.T, setup func(db *sql.DB)) *Repository {
	t.Helper()
	db := setupTestDB(t)
	setup(db)
	return NewRepository(db)
}

func totalSongs(t *testing.T, db *sql.DB) int {
	t.Helper()
	var count int
	err := db.QueryRow("SELECT COUNT(*) FROM songs;").Scan(&count)
	if err != nil {
		t.Errorf("query error: %v", err)
	}
	return count
}

func getByFileID(t *testing.T, db *sql.DB, id string) Song {
	t.Helper()
	var s Song
	err := db.QueryRow(
		"SELECT fileID, day, month FROM songs WHERE fileID = ?",
		id).Scan(&s.FileID, &s.Day, &s.Month)
	if err != nil {
		t.Errorf("query error: %v", err)
	}
	return s
}
