package bot

import (
	"database/sql"
	"log"

	"based-bot/config"
	"based-bot/internal/song"

	tgbot "github.com/go-telegram-bot-api/telegram-bot-api/v5"
)

func Start(cfg config.Config, db *sql.DB) {
	bot, err := tgbot.NewBotAPI(cfg.Token)
	if err != nil {
		log.Fatalf("Error trying to init bot: %v", err)
	}
	bot.Debug = true
	updateConfig := tgbot.NewUpdate(0)
	updateConfig.Timeout = 30

	updates := bot.GetUpdatesChan(updateConfig)

	repo := song.NewRepository(db)
	handler := newHandler(bot, cfg, repo)

	go scheduler(handler)
	pooling(handler, updates)
}
