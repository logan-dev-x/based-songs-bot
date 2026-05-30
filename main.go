package main

import (
	"log"
	"os"

	"based-bot/bot"
	"based-bot/config"
	"based-bot/database"

	_ "github.com/mattn/go-sqlite3"
)

func main() {
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
