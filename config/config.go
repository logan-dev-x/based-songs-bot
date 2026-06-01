package config

import (
	"log"
	"os"
	"strconv"

	"github.com/joho/godotenv"
)

type Config struct {
	Token     string
	DBPath    string
	AdminID   int64
	ChannelID int64
	Caption   string
}

func Load() Config {
	_ = godotenv.Load()

	token := os.Getenv("TELEGRAM_APITOKEN")
	if token == "" {
		log.Fatal("TELEGRAM_APITOKEN not defined")
	}

	dbPath := os.Getenv("DB_PATH")
	if dbPath == "" {
		log.Fatal("DBPath not defined. Using default path")
		dbPath = "../data.db"
	}

	adminID, err := strconv.ParseInt(os.Getenv("ADMIN_ID"), 10, 64)
	if err != nil {
		log.Fatalf("Error converting AdminID: %v", err)
	}

	channelID, err := strconv.ParseInt(os.Getenv("CHANNEL_ID"), 10, 64)
	if err != nil {
		log.Fatalf("Error converting ChannelID: %v", err)
	}

	caption := os.Getenv("CAPTION")

	return Config{
		Token:     token,
		DBPath:    dbPath,
		AdminID:   adminID,
		ChannelID: channelID,
		Caption:   caption,
	}
}
