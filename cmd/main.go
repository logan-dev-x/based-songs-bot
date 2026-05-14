package main

import (
	tgbot "github.com/go-telegram-bot-api/telegram-bot-api/v5"
)

func main() {
	bot, err := tgbot.NewBotAPI("6972468650:AAFTCAxRiawemXha0KH4fhwgamAI0yxpQDY")
	if err != nil {
		panic(err)
	}

	bot.Debug = true

	updateConfig := tgbot.NewUpdate(0)

	updateConfig.Timeout = 30

	updates := bot.GetUpdatesChan(updateConfig)

	for update := range updates {
		if update.Message.Audio == nil {
			continue
		}

		msg := tgbot.NewMessage(update.Message.Chat.ID, "audio")

		msg.ReplyToMessageID = update.Message.MessageID

		if _, err := bot.Send(msg); err != nil {
			panic(err)
		}
	}
}
