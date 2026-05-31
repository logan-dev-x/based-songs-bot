package bot

import "testing"

func Test_extractScheduleDate(t *testing.T) {
	tests := []struct {
		name string // description of this test case
		// Named input parameters for target function.
		caption   string
		wantDay   int
		wantMonth int
		wantErr   bool
	}{
		{
			"test with right values",
			"25/12",
			25,
			12,
			false,
		},
		{
			"date with text together",
			"random foo 26/02",
			26,
			2,
			false,
		},
		{
			"day and month with only one digit",
			"2/6",
			2,
			6,
			false,
		},
		{
			"just text, without date",
			"foo and boo",
			0,
			0,
			true,
		},
		{
			"invalid day",
			"32/06",
			0,
			0,
			true,
		},
		{
			"invalid month",
			"2/13",
			0,
			0,
			true,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gotDay, gotMonth, gotErr := extractScheduleDate(tt.caption)

			if gotErr != nil {
				if !tt.wantErr {
					t.Errorf("extractScheduleDate() failed: %v", gotErr)
				}
				return
			}
			if tt.wantErr {
				t.Fatal("extractScheduleDate() succeeded unexpectedly")
			}

			if gotDay != tt.wantDay {
				t.Errorf("got day = %v, want %v", gotDay, tt.wantDay)
			}
			if gotMonth != tt.wantMonth {
				t.Errorf("got month = %v, want %v", gotMonth, tt.wantMonth)
			}
		})
	}
}
