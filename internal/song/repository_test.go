package song

import (
	"database/sql"
	"reflect"
	"testing"

	_ "github.com/mattn/go-sqlite3"
)

func TestRepository_Save(t *testing.T) {
	tests := []struct {
		name      string
		setup     func(db *sql.DB)
		song      Song
		wantErr   bool
		wantCount int
	}{
		{
			"save first song",
			func(db *sql.DB) {
			},
			Song{
				FileID: "mock_1",
				Day:    2,
				Month:  5,
			},
			false,
			1,
		},
		{
			"error when song already exists",
			func(db *sql.DB) {
				insertMock(t, db, "mock_2", 2, 6)
			},
			Song{
				FileID: "mock_2",
				Day:    2,
				Month:  5,
			},
			true,
			1,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			r := newTestRepository(t, tt.setup)

			gotErr := r.Save(tt.song)
			if gotErr != nil {
				if !tt.wantErr {
					t.Fatalf("Save() returned an unexpected error: %v", gotErr)
				}
				return
			}

			if tt.wantErr && gotErr == nil {
				t.Fatalf("expect an error but got nil, song: %v", tt.song)
			}

			total := totalSongs(t, r.db)
			if tt.wantCount != total {
				t.Errorf("expect %d songs in DB, but got %d", tt.wantCount, total)
			}
		})
	}
	// rows, err := db.Query("SELECT * FROM songs WHERE day = ? AND month = ?;", 2, 5)
	// if err != nil {
	// 	t.Errorf("Query returned an unexpected error: %v", err)
	// }
	// defer rows.Close()
	// var result Song
	// for rows.Next() {
	// 	if err := rows.Scan(&result.FileID, &result.Day, &result.Month); err != nil {
	// 		t.Errorf("Scan returned an error: %v", err)
	// 	}
	// }
	// if result.FileID != mockSong.FileID {
	// 	t.Errorf("Expected %s, but received: %s", mockSong.FileID, result.FileID)
	// }
	// if result.Day != mockSong.Day {
	// 	t.Errorf("Expected %d, but received: %d", mockSong.Day, result.Day)
	// }
	// if result.Month != mockSong.Month {
	// 	t.Errorf("Expected %d, but received: %d", mockSong.Month, result.Month)
	// }
}

func TestRepository_GetByDate(t *testing.T) {
	tests := []struct {
		name string // description of this test case
		// Named input parameters for receiver constructor.
		setup func(db *sql.DB)
		// Named input parameters for target function.
		day     int
		month   int
		want    []Song
		wantErr bool
	}{
		{
			"existend Song",
			func(db *sql.DB) {
				insertMock(t, db, "mock_1234", 1, 3)
			},
			1,
			3,
			[]Song{
				{
					FileID: "mock_1234",
					Day:    1,
					Month:  3,
				},
			},
			false,
		},

		{
			"song not stored",
			func(db *sql.DB) {},
			2,
			4,
			[]Song{},
			false,
		},

		{
			"muiltiply Songs",
			func(db *sql.DB) {
				insertMock(t, db, "mock_1234", 1, 3)
				insertMock(t, db, "mock_5432", 1, 3)
			},
			1,
			3,
			[]Song{
				{
					FileID: "mock_1234",
					Day:    1,
					Month:  3,
				},
				{
					FileID: "mock_5432",
					Day:    1,
					Month:  3,
				},
			},
			false,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			r := newTestRepository(t, tt.setup)

			got, gotErr := r.GetByDate(tt.day, tt.month)
			if gotErr != nil {
				if !tt.wantErr {
					t.Errorf("GetByDate() failed: %v", gotErr)
				}
				return
			}
			if tt.wantErr {
				t.Fatal("GetByDate() succeeded unexpectedly")
			}
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("got = %v, want %v, got == nil? %v", got, tt.want, got == nil)
			}
		})
	}
}

func TestRepository_Delete(t *testing.T) {
	tests := []struct {
		name string // description of this test case
		// Named input parameters for receiver constructor.
		setup func(db *sql.DB)
		// Named input parameters for target function.
		fileID    string
		wantErr   bool
		wantCount int
	}{
		{
			"delete existing song",
			func(db *sql.DB) {
				insertMock(t, db, "mock_file_123", 1, 3)
			},
			"mock_file_123",
			false,
			0,
		},
		{
			"delete non-existing song",
			func(db *sql.DB) {
			},
			"mock_file_123",
			false,
			0,
		},
		{
			"delete only target song",
			func(db *sql.DB) {
				insertMock(t, db, "mock_file_1", 1, 3)
				insertMock(t, db, "mock_file_2", 2, 3)
				insertMock(t, db, "mock_file_3", 2, 3)
			},
			"mock_file_3",
			false,
			2,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			r := newTestRepository(t, tt.setup)

			gotErr := r.Delete(tt.fileID)
			if gotErr != nil {
				if !tt.wantErr {
					t.Errorf("Delete() failed: %v", gotErr)
				}
				return
			}
			if tt.wantErr {
				t.Fatal("Delete() succeeded unexpectedly")
			}

			var count int

			err := r.db.QueryRow("SELECT COUNT(*) FROM songs WHERE fileID = ?", tt.fileID).Scan(&count)
			if err != nil {
				t.Fatalf("count target song: %v", err)
			}
			if count != 0 {
				t.Fatal("target song still exists")
			}

			count = totalSongs(t, r.db)
			if count != tt.wantCount {
				t.Errorf(
					"expect %d reamining, got %d",
					tt.wantCount,
					count,
				)
			}
		})
	}
}
