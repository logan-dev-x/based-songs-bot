package main

import (
	"database/sql"
	"log"
)

func openDB() *sql.DB {
	db, err := sql.Open("sqlite3", "./data.db")
	if err != nil {
		log.Println(err)
	}
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
	if err != nil {
		log.Println(err)
	}
}

func getScheduledSongs() []Song {
	db := openDB()
	defer db.Close()

	rows, err := db.Query("SELECT fileId, day, month FROM SONGS")
	if err != nil {
		log.Println(err)
	}

	var songs []Song

	for rows.Next() {
		var song Song
		if err := rows.Scan(&song.FileID, &song.Day, &song.Month); err != nil {
			log.Println(err)
		}
		songs = append(songs, song)
	}

	return songs
}
