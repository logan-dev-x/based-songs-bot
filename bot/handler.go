package bot

import (
	"fmt"
	"log"

	"based-bot/config"
	"based-bot/internal/song"

	tgbot "github.com/go-telegram-bot-api/telegram-bot-api/v5"
)

type Handler struct {
	bot  *tgbot.BotAPI
	cfg  config.Config
	repo *song.Repository
}

func newHandler(
	bot *tgbot.BotAPI,
	cfg config.Config,
	repo *song.Repository,
) *Handler {
	return &Handler{bot, cfg, repo}
}

func (h *Handler) handleCommand(cmd string) {
	switch cmd {
	case "list":
		h.handleListCmd()
	default:
		h.sendMsg("Comando não reconhecido.")
	}
}

func (h *Handler) handleListCmd() {
	songs, err := h.repo.GetAll()
	if err != nil {
		h.sendMsg(err.Error())
		return
	}

	if len(songs) == 0 {
		h.sendMsg("Sem músicas agendadas")
		return
	}

	for _, s := range songs {
		audio := tgbot.NewAudio(h.cfg.AdminID, tgbot.FileID(s.FileID))
		audio.Caption = fmt.Sprintf("Dia: %d\nMês: %d", s.Day, s.Month)

		sendNowBtn := tgbot.NewInlineKeyboardButtonData("▶️Enviar", fmt.Sprintf("sendNow: %d", s.ID))
		deleteBtn := tgbot.NewInlineKeyboardButtonData("❌Excluir", fmt.Sprintf("delete: %d", s.ID))

		audio.ReplyMarkup = tgbot.NewInlineKeyboardMarkup(
			tgbot.NewInlineKeyboardRow(sendNowBtn, deleteBtn))

		if _, err := h.bot.Send(audio); err != nil {
			log.Printf("Error executing bot.Send(Audio): %v", err)
		}
	}
}

func (h *Handler) handleSongUpload(update tgbot.Update) {
	day, month, err := extractScheduleDate(update.Message.Caption)
	if err != nil {
		h.sendMsg(err.Error())
		return
	}
	s := song.Song{
		FileID: update.Message.Audio.FileID,
		Day:    day,
		Month:  month,
	}

	if err := h.repo.Save(s); err != nil {
		h.sendMsg(err.Error())
		return
	}

	h.sendMsg("Música agendada com sucesso!")
}

func (h *Handler) sendMsg(text string) {
	msg := tgbot.NewMessage(h.cfg.AdminID, text)
	if _, err := h.bot.Send(msg); err != nil {
		log.Printf("Error executing bot.Send(Text): %v", err)
	}
}

func (h *Handler) sendAudio(chatID int64, fileID, caption string) {
	audio := tgbot.NewAudio(chatID, tgbot.FileID(fileID))
	audio.Caption = caption

	if _, err := h.bot.Send(audio); err != nil {
		log.Printf("Error executing bot.Send(Audio): %v", err)
	}
}

func (h *Handler) sendAudioNow(id int) {
	song, err := h.repo.GetByID(id)
	if err != nil {
		h.sendMsg(err.Error())
	}
	h.sendAudio(h.cfg.ChannelID, song.FileID, h.cfg.Caption)
}

func (h *Handler) deleteSong(id int) {
	err := h.repo.Delete(id)
	if err != nil {
		h.sendMsg(err.Error())
		return
	}
	h.sendMsg("Música deletada")
}
