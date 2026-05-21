package main

import (
	"database/sql"
	"os"
	"regexp"
	"time"

	tgbot "github.com/go-telegram-bot-api/telegram-bot-api/v5"
	"github.com/joho/godotenv"
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
	db := setupDB()
	err := godotenv.Load()
	if err != nil {
		panic(err)
	}
	bot, err := tgbot.NewBotAPI(os.Getenv("TELEGRAM_APITOKEN"))
	if err != nil {
		panic(err)
	}

	// channelId := -1001733966614

	bot.Debug = true

	updateConfig := tgbot.NewUpdate(0)

	updateConfig.Timeout = 30

	updates := bot.GetUpdatesChan(updateConfig)

	// go scheduler(bot)

	for update := range updates {
		if update.Message.Audio == nil {
			continue
		}

		// audioMsg := tgbot.NewAudio(int64(channelId), tgbot.FileID(update.Message.Audio.FileID))
		// audioMsg.Caption = "🗿@BasedSongs"
		r, _ := regexp.Compile(`([0-9]{2})/([0-9]{2})`)
		date := r.FindStringSubmatch(update.Message.Caption)
		_, err = db.Exec("insert into schudulers(fileId, day, month) values (?, ?, ?)", update.Message.Audio.FileID, date[1], date[2])
	}
}

func setupDB() *sql.DB {
	db, err := sql.Open("sqlite3", "./data.db")
	if err != nil {
		panic(err)
	}
	defer db.Close()
	sqlStmt := `
	CREATE TABLE IF NOT EXISTS musics (
		fileId TEXT NOT NULL PRIMARY KEY,
		day TEXT NOT NULL,
		month TEXT NOT NULL
	);`
	_, err = db.Exec(sqlStmt)
	if err != nil {
		panic(err)
	}
	return db
}
