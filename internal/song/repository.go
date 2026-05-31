package song

import (
	"database/sql"
	"errors"
)

type Repository struct {
	db *sql.DB
}

func NewRepository(db *sql.DB) *Repository {
	return &Repository{db: db}
}

func (r *Repository) Save(s Song) error {
	var count int
	err := r.db.QueryRow("SELECT COUNT(*) FROM songs WHERE file_id = ?;", s.FileID).Scan(&count)
	if err != nil {
		return err
	}
	if count != 0 {
		return errors.New("an song with this file_id already exists")
	}
	query := "INSERT INTO songs (file_id, day, month) VALUES (?, ?, ?)"
	_, err = r.db.Exec(query, s.FileID, s.Day, s.Month)
	return err
}

func (r *Repository) GetAll() ([]Song, error) {
	rows, err := r.db.Query("SELECT id, file_id, day, month FROM songs")
	if err != nil {
		return nil, err
	}

	songs := []Song{}
	for rows.Next() {
		var s Song
		err = rows.Scan(&s.ID, &s.FileID, &s.Day, &s.Month)
		if err != nil {
			return nil, err
		}
		songs = append(songs, s)
	}

	return songs, nil
}

func (r *Repository) GetByDate(day int, month int) ([]Song, error) {
	rows, err := r.db.Query("SELECT id, file_id, day, month FROM songs WHERE day = ? AND month = ?", day, month)
	if err != nil {
		return nil, err
	}

	songs := []Song{}
	for rows.Next() {
		var s Song
		err = rows.Scan(&s.ID, &s.FileID, &s.Day, &s.Month)
		if err != nil {
			return nil, err
		}
		songs = append(songs, s)
	}

	return songs, nil
}

func (r *Repository) Delete(id int) error {
	_, err := r.db.Exec("DELETE FROM songs WHERE id = ?", id)
	return err
}

func (r *Repository) GetByID(id int) (Song, error) {
	s := Song{}
	err := r.db.QueryRow(
		"SELECT id, file_id, day,month FROM songs WHERE id = ?;", id).Scan(
		&s.ID, &s.FileID, &s.Day, &s.Month)
	return s, err
}
