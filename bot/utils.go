package bot

import (
	"errors"
	"regexp"
	"strconv"
)

func extractScheduleDate(caption string) (int, int, error) {
	r, _ := regexp.Compile(`(\d{2})\/(\d{2})`)
	matches := r.FindStringSubmatch(caption)
	if len(matches) < 3 {
		return 0, 0, errors.New("não foi possível reconhecer a data do agendamento")
	}

	day, _ := strconv.Atoi(matches[1])
	month, _ := strconv.Atoi(matches[2])

	return day, month, nil
}
