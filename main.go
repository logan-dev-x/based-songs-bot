package main

import (
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

		db := openDB()
		defer db.Close()

		// audioMsg := tgbot.NewAudio(int64(channelId), tgbot.FileID(update.Message.Audio.FileID))
		// audioMsg.Caption = "🗿@BasedSongs"

		day, month := getScheduleDate(update.Message.Caption)

		_, err = db.Exec("INSERT INTO songs (fileId, day, month) VALUES (?, ?, ?)", update.Message.Audio.FileID, day, month)
		if err != nil {
			panic(err)
		}
	}
}

func getScheduleDate(caption string) (string, string) {
	r, _ := regexp.Compile(`([0-9]{2})/([0-9]{2})`)
	date := r.FindStringSubmatch(caption)
	return date[1], date[2]
}

func handleCommand(update tgbot.Update, bot tgbot.BotAPI) {
	chatId := update.Message.Chat.ID

	switch update.Message.Command() {
	case "list":
		handleListCmd(chatId, bot)
	default:
		handleDefaultCmd(chatId, bot)
	}
}

func handleDefaultCmd(chatId int64, bot tgbot.BotAPI) {
	msg := tgbot.NewMessage(chatId, "Comando não reconhecido.")
	if _, err := bot.Send(msg); err != nil {
		panic(err)
	}
}

func handleListCmd(chatId int64, bot tgbot.BotAPI) {
	for _, song := range getScheduledSongs() {
		audio := tgbot.NewAudio(chatId, tgbot.FileID(song.FileID))
		audio.Caption = fmt.Sprintf("Dia: %s\nMês: %s", song.Day, song.Month)

		if _, err := bot.Send(audio); err != nil {
			panic(err)
		}
	}
}
