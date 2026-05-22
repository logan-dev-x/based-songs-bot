package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"time"

	tgbot "github.com/go-telegram-bot-api/telegram-bot-api/v5"
	"github.com/joho/godotenv"
	_ "github.com/mattn/go-sqlite3"
)

func main() {
	setupLogger()
	setupDB()

	err := godotenv.Load()
	checkFatal(err)

	token := os.Getenv("TELEGRAM_APITOKEN")
	bot, err := tgbot.NewBotAPI(token)
	checkFatal(err)

	// channelId := -1001733966614

	bot.Debug = true

	updateConfig := tgbot.NewUpdate(0)
	updateConfig.Timeout = 30

	updates := bot.GetUpdatesChan(updateConfig)

	go scheduler(bot)
	pooling(updates, bot)
}

func setupLogger() {
	file, err := os.OpenFile("log.txt", os.O_CREATE|os.O_WRONLY, 0o664)
	checkFatal(err)

	log.SetOutput(file)
	log.SetFlags(log.LstdFlags | log.Lshortfile)
}

func scheduler(bot *tgbot.BotAPI) {
	for {
		if !thereAreSchedules() {
			fmt.Println("not shecules")
			time.Sleep(5 * time.Second)
			continue
		}

		fmt.Println("there're shecules")

		loc, _ := time.LoadLocation("America/Sao_Paulo")
		songs := getScheduledSongs()

		now := time.Now().In(loc)
		for _, song := range songs {
			if day, _ := strconv.Atoi(song.Day); day == now.Day() && now.Hour() >= 22 {
				audio := tgbot.NewAudio(1071520377, tgbot.FileID(song.FileID))
				audio.Caption = "🗿@BasedSongs"
				_, err := bot.Send(audio)
				check(err)
			}
		}

		time.Sleep(5 * time.Second)
	}
}

func pooling(updates tgbot.UpdatesChannel, bot *tgbot.BotAPI) {
	for update := range updates {
		if update.Message.IsCommand() {
			handleCommand(update, bot)
		}
		if update.Message.Audio == nil {
			continue
		}

		db := openDB()
		defer db.Close()

		day, month := getScheduleDate(update.Message.Caption)

		_, err := db.Exec("INSERT INTO songs (fileId, day, month) VALUES (?, ?, ?)", update.Message.Audio.FileID, day, month)
		check(err)
	}
}

func getScheduleDate(caption string) (string, string) {
	r, _ := regexp.Compile(`([0-9]{2})/([0-9]{2})`)
	date := r.FindStringSubmatch(caption)
	return date[1], date[2]
}

func handleCommand(update tgbot.Update, bot *tgbot.BotAPI) {
	chatId := update.Message.Chat.ID

	switch update.Message.Command() {
	case "list":
		handleListCmd(chatId, bot)
	default:
		handleDefaultCmd(chatId, bot)
	}
}

func handleDefaultCmd(chatId int64, bot *tgbot.BotAPI) {
	msg := tgbot.NewMessage(chatId, "Comando não reconhecido.")
	_, err := bot.Send(msg)
	check(err)
}

func handleListCmd(chatId int64, bot *tgbot.BotAPI) {
	for _, song := range getScheduledSongs() {
		audio := tgbot.NewAudio(chatId, tgbot.FileID(song.FileID))
		audio.Caption = fmt.Sprintf("Dia: %s\nMês: %s", song.Day, song.Month)

		_, err := bot.Send(audio)
		check(err)
	}
}
