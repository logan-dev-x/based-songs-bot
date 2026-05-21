package main

import (
	"database/sql"
	"fmt"
	"os"
	"regexp"
	"time"

	tgbot "github.com/go-telegram-bot-api/telegram-bot-api/v5"
	"github.com/joho/godotenv"
	_ "github.com/mattn/go-sqlite3"
)

func scheduler(bot *tgbot.BotAPI) {
	for {
		println("sewrching for schedules")

		msg := tgbot.NewMessage(1071520377, "hi")
		bot.Send(msg)

		time.Sleep(2 * time.Second)
	}
}

func main() {
	setupDB()
	err := godotenv.Load()
	if err != nil {
		panic(err)
	}
	token := os.Getenv("TELEGRAM_APITOKEN")
	bot, er := tgbot.NewBotAPI(token)
	if er != nil {
		panic(er)
	}

	// channelId := -1001733966614

	bot.Debug = true

	updateConfig := tgbot.NewUpdate(0)

	updateConfig.Timeout = 30

	updates := bot.GetUpdatesChan(updateConfig)

	// go scheduler(bot)

	for update := range updates {
		if update.Message.IsCommand() {
			handleCommand(update, *bot)
		}
		if update.Message.Audio == nil {
			continue
		}

		// audioMsg := tgbot.NewAudio(int64(channelId), tgbot.FileID(update.Message.Audio.FileID))
		// audioMsg.Caption = "🗿@BasedSongs"
		r, _ := regexp.Compile(`([0-9]{2})/([0-9]{2})`)
		date := r.FindStringSubmatch(update.Message.Caption)
		db, err := sql.Open("sqlite3", "./data.db")
		if err != nil {
			panic(err)
		}
		defer db.Close()
		_, err = db.Exec("INSERT INTO songs (fileId, day, month) VALUES (?, ?, ?)", update.Message.Audio.FileID, date[1], date[2])
		if err != nil {
			panic(err)
		}
	}
}

func handleCommand(update tgbot.Update, bot tgbot.BotAPI) {
	chatId := update.Message.Chat.ID
	switch update.Message.Command() {
	case "list":
		songs := getScheduledSongs()
		for _, song := range songs {
			audio := tgbot.NewAudio(chatId, tgbot.FileID(song[0]))
			audio.Caption = fmt.Sprintf("Dia: %s\nMês: %s", song[1], song[2])
			if _, err := bot.Send(audio); err != nil {
				panic(err)
			}
		}
	default:
		msg := tgbot.NewMessage(chatId, "Comando não reconhecido.")
		if _, err := bot.Send(msg); err != nil {
			panic(err)
		}
	}
}

func getScheduledSongs() [][]string {
	db, err := sql.Open("sqlite3", "./data.db")
	if err != nil {
		panic(err)
	}
	var songs [][]string
	rows, err := db.Query("select * from songs")
	for rows.Next() {
		var fileId, day, month string
		if err := rows.Scan(&fileId, &day, &month); err != nil {
			panic(err)
		}
		songs = append(songs, []string{fileId, day, month})
	}
	return songs
}

func setupDB() *sql.DB {
	db, err := sql.Open("sqlite3", "./data.db")
	if err != nil {
		panic(err)
	}
	defer db.Close()
	sqlStmt := `
	CREATE TABLE IF NOT EXISTS songs (
		fileId TEXT NOT NULL,
		day TEXT NOT NULL,
		month TEXT NOT NULL
	);`
	_, err = db.Exec(sqlStmt)
	if err != nil {
		panic(err)
	}
	return db
}
