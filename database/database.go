package database

import (
	"database/sql"
	"log"
)

func InitDB() *sql.DB {
	db, err := sql.Open("sqlite3", "./data.db")
	if err != nil {
		log.Fatalf("Trying to open db error: %v", err)
	}

	migration := `
	CREATE TABLE IF NOT EXISTS songs (
		fileID TEXT NOT NULL,
		day INTEGER NOT NULL,
		month INTEGER NOT NULL
	);`

	_, err = db.Exec(migration)
	if err != nil {
		log.Fatalf("Error while creating migrations: %v", err)
	}

	return db
}
