package bot

import (
	"log"

	tgbot "github.com/go-telegram-bot-api/telegram-bot-api/v5"
)

func pooling(h *Handler, updates tgbot.UpdatesChannel) {
	for update := range updates {
		if update.CallbackQuery != nil {
			h.sendMsg(update.CallbackQuery.Data)
			continue
		}
		if update.Message == nil {
			continue
		}
		if update.Message.From.ID != h.cfg.AdminID {
			log.Printf("Tentativa de acesso bloqueado. ID : %v", update.Message.Chat.ID)
			continue
		}

		if update.Message.IsCommand() {
			h.handleCommand(update.Message.Command())
			continue
		}

		if update.Message.Audio != nil {
			h.handleSongUpload(update)
			continue
		}
	}
}
