package main

import (
	"database/sql"
)

func openDB() *sql.DB {
	db, err := sql.Open("sqlite3", "./data.db")
	check(err)
	return db
}

func setupDB() {
	db := openDB()
	defer db.Close()

	sqlStmt := `
	CREATE TABLE IF NOT EXISTS songs (
		fileId TEXT NOT NULL,
		day TEXT NOT NULL,
		month TEXT NOT NULL
	);`

	_, err := db.Exec(sqlStmt)
	checkFatal(err)
}

func getScheduledSongs() []Song {
	db := openDB()
	defer db.Close()

	rows, err := db.Query("SELECT fileId, day, month FROM SONGS")
	check(err)

	var songs []Song

	for rows.Next() {
		var song Song
		err := rows.Scan(&song.FileID, &song.Day, &song.Month)
		check(err)
		songs = append(songs, song)
	}

	return songs
}

func thereAreSchedules() bool {
	db := openDB()
	defer db.Close()

	var count int
	err := db.QueryRow("SELECT COUNT(*) FROM songs").Scan(&count)
	checkFatal(err)

	return count > 0
}
