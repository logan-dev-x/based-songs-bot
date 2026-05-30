package song

import (
	"database/sql"
	"testing"

	_ "github.com/mattn/go-sqlite3"
)

func setupTestDB(t *testing.T) *sql.DB {
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

func TestSave(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	repo := NewRepository(db)
	mockSong := Song{
		FileID: "audi_1234_mock",
		Day:    2,
		Month:  5,
	}

	err := repo.Save(mockSong)
	if err != nil {
		t.Errorf("Save() returned an unexpected error: %v", err)
	}

	rows, err := db.Query("SELECT * FROM songs WHERE day = ? AND month = ?;", 2, 5)
	if err != nil {
		t.Errorf("Query returned an unexpected error: %v", err)
	}
	defer rows.Close()
	var result Song
	for rows.Next() {
		if err := rows.Scan(&result.FileID, &result.Day, &result.Month); err != nil {
			t.Errorf("Scan returned an error: %v", err)
		}
	}
	if result.FileID != mockSong.FileID {
		t.Errorf("Expected %s, but received: %s", mockSong.FileID, result.FileID)
	}
	if result.Day != mockSong.Day {
		t.Errorf("Expected %d, but received: %d", mockSong.Day, result.Day)
	}
	if result.Month != mockSong.Month {
		t.Errorf("Expected %d, but received: %d", mockSong.Month, result.Month)
	}
}
