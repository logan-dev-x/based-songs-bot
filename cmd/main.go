package main

import (
	"os"

	tgbot "github.com/go-telegram-bot-api/telegram-bot-api/v5"
	"github.com/joho/godotenv"
)

func main() {
	err := godotenv.Load()
	if err != nil {
		panic(err)
	}
	bot, err := tgbot.NewBotAPI(os.Getenv("TELEGRAM_APITOKEN"))
	if err != nil {
		panic(err)
	}

	channelId := -1001733966614

	bot.Debug = true

	updateConfig := tgbot.NewUpdate(0)

	updateConfig.Timeout = 30

	updates := bot.GetUpdatesChan(updateConfig)

	for update := range updates {
		if update.Message.Audio == nil {
			continue
		}

		audioMsg := tgbot.NewAudio(int64(channelId), tgbot.FileID(update.Message.Audio.FileID))
		audioMsg.Caption = "🗿@BasedSongs"

		if _, err := bot.Send(audioMsg); err != nil {
			panic(err)
		}
	}
}
