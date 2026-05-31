package bot

import (
	"errors"
	"fmt"
	"regexp"
	"strconv"
)

func extractScheduleDate(caption string) (int, int, error) {
	r, _ := regexp.Compile(`(\d{1,2})\/(\d{1,2})`)
	matches := r.FindStringSubmatch(caption)
	if len(matches) < 3 {
		return 0, 0, errors.New("não foi possível reconhecer a data do agendamento")
	}

	day, _ := strconv.Atoi(matches[1])
	month, _ := strconv.Atoi(matches[2])

	if day > 31 || day < 1 {
		return 0, 0, errors.New("dia inválido")
	}
	if month > 12 || month < 1 {
		return 0, 0, errors.New("mês inválido")
	}

	return day, month, nil
}

func extractActionAndID(content string) (string, int, error) {
	r, _ := regexp.Compile(`(\w+): (\d+)`)
	matches := r.FindStringSubmatch(content)
	if len(matches) < 3 {
		return "", 0, fmt.Errorf("não foi possível identificar os valores de aćão em: %s", content)
	}

	id, _ := strconv.Atoi(matches[2])

	return matches[1], id, nil
}
