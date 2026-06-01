package main

import (
	"log"
	"net/http"
	"os"

	"based-bot/bot"
	"based-bot/config"
	"based-bot/database"

	_ "github.com/mattn/go-sqlite3"
)

func main() {
	go func() {
		// O Render injeta automaticamente a variável PORT (geralmente 10000)
		port := os.Getenv("PORT")
		if port == "" {
			port = "8080" // Porta padrão local se rodar no seu PC
		}

		// Rota simples que responde 200 OK para o Render saber que o bot está vivo
		http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
			w.WriteHeader(http.StatusOK)
			w.Write([]byte("Bot ativo!"))
		})

		log.Printf("Servidor de verificação do Render rodando na porta %s", port)
		if err := http.ListenAndServe(":"+port, nil); err != nil {
			log.Printf("Erro no servidor HTTP: %v", err)
		}
	}()
	setupLogger()

	cfg := config.Load()

	db := database.InitDB()
	defer db.Close()

	log.Println("Iniciando o bot...")
	bot.Start(cfg, db)
}

func setupLogger() {
	file, err := os.OpenFile("log.txt", os.O_CREATE|os.O_WRONLY, 0o664)
	if err != nil {
		log.Fatalf("logger error: %v", err)
	}
	log.SetOutput(file)
	log.SetFlags(log.LstdFlags | log.Lshortfile)
}
