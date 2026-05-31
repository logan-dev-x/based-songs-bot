package bot

import (
	"testing"

	"based-bot/internal/song"

	tgbot "github.com/go-telegram-bot-api/telegram-bot-api/v5"
	_ "github.com/mattn/go-sqlite3"
)

func TestHandler_handleSongUpload(t *testing.T) {
	tests := []struct {
		name   string // description of this test case
		update tgbot.Update
	}{
		{
			"ensure sended data will be saved",
			tgbot.Update{
				Message: &tgbot.Message{
					From: &tgbot.User{ID: 12345}, // Mesmo AdminID da configuração
					Audio: &tgbot.Audio{
						FileID: "audio_id_de_teste",
					},
					Caption: "25/12", // Legenda que sua função extractScheduleDate vai ler
				},
			},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			h, db := setupHandlerTest(t)

			h.handleSongUpload(tt.update)

			var got song.Song
			err := db.QueryRow("SELECT fileID, day, month FROM songs LIMIT 1").Scan(&got.FileID, &got.Day, &got.Month)
			if err != nil {
				t.Fatalf("query error: %v", err)
			}

			if got.FileID != "audio_id_de_teste" || got.Day != 25 || got.Month != 12 {
				t.Fatalf("unexpect values: %s %d %d", got.FileID, got.Day, got.Month)
			}
		})
	}
}
