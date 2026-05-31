package bot

import (
	"database/sql"
	"net/http"
	"net/http/httptest"
	"testing"

	"based-bot/config"
	"based-bot/internal/song"

	tgbot "github.com/go-telegram-bot-api/telegram-bot-api/v5"
)

func setupHandlerTest(t *testing.T) (*Handler, *sql.DB) {
	t.Helper()
	// 1. Criamos um servidor HTTP falso para simular a API do Telegram
	// Isso impede que o bot tente conectar de verdade na internet durante o teste
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		// Retorna uma resposta padrão de sucesso que a biblioteca do Telegram espera
		w.Write([]byte(`{"ok":true,"result":{"message_id":1}}`))
	}))

	// 2. Criamos o bot apontando para o nosso servidor interno falso
	bot, err := tgbot.NewBotAPIWithAPIEndpoint("fake-token", server.URL+"/bot%s/%s")
	if err != nil {
		t.Fatalf("Erro ao criar bot falso: %v", err)
	}

	db := song.SetupTestDB(t)
	r := song.NewRepository(db)

	cfg := config.Config{AdminID: 12345}

	// 5. Retornamos o Handler pronto para o teste
	return newHandler(bot, cfg, r), db
}
