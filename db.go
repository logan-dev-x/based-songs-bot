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

	confgiSql := `
	CREATE TABLE if NOT EXISTS config (
		key TEXT NOT NULL,
		value TEXT NOT NULL
	);`
	_, err := db.Exec(confgiSql)
	checkFatal(err)

	songsSql := `
	CREATE TABLE IF NOT EXISTS songs (
		fileId TEXT NOT NULL,
		day TEXT NOT NULL,
		month TEXT NOT NULL
	);`

	_, err = db.Exec(songsSql)
	checkFatal(err)
}

func getScheduledSongs() []Song {
	db := openDB()
	defer db.Close()

	rows, err := db.Query("SELECT fileId, day, month FROM songs")
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

func deleteSong(fileID string) {
	db := openDB()
	defer db.Close()

	_, err := db.Exec("DELETE FROM songs WHERE fileId = ?", fileID)
	check(err)
}

func addConfig(key, value string) error {
	db := openDB()
	defer db.Close()

	_, err := db.Exec("INSERT INTO config (key, value) VALUES (?,?);", key, value)
	return err
}
