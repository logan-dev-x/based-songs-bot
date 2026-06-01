package bot

import (
	"fmt"

	tgbot "github.com/go-telegram-bot-api/telegram-bot-api/v5"
)

func pooling(h *Handler, updates tgbot.UpdatesChannel) {
	for update := range updates {
		if update.CallbackQuery != nil {
			action, id, err := extractActionAndID(update.CallbackQuery.Data)
			if err != nil {
				h.sendMsg(err.Error())
			}

			switch action {
			case "sendNow":
				h.sendAudioNow(id)
			case "delete":
				h.deleteSong(id)
			}
			continue
		}
		if update.Message == nil {
			continue
		}
		if update.Message.From.ID != h.cfg.AdminID {
			h.sendMsg(fmt.Sprintf("Tentativa de acesso bloqueado. ID : %v", update.Message.Chat.ID))
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
