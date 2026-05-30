package bot

import (
	"log"
	"time"
)

func scheduler(h *Handler) {
	loc, err := time.LoadLocation("America/Sao_Paulo")
	if err != nil {
		log.Printf("Error get zone time: %v", err)
		loc = time.Local
	}

	checkAndSend(h, loc)

	ticker := time.NewTicker(1 * time.Hour)
	defer ticker.Stop()

	for range ticker.C {
		checkAndSend(h, loc)
	}
}

func checkAndSend(h *Handler, loc *time.Location) {
	now := time.Now().In(loc)
	hour := now.Hour()

	if hour < 17 {
		return
	}

	day := now.Day()
	month := int(now.Month())

	songs, err := h.repo.GetByDate(day, month)
	if err != nil {
		h.sendMsg(err.Error())
	}

	for _, song := range songs {
		h.sendAudio(song.FileID, h.cfg.Caption)

		err = h.repo.Delete(song.FileID)
		if err != nil {
			h.sendMsg(err.Error())
		}
	}
}
