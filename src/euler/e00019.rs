fn is_leap_year(year: u32) -> bool {
    if year % 400 == 0 {
        true
    } else if year % 100 == 0 {
        false
    } else {
        year % 4 == 0
    }
}

enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

impl Month {
    fn num(&self) -> u8 {
        match self {
            Month::Jan => 1,
            Month::Feb => 2,
            Month::Mar => 3,
            Month::Apr => 4,
            Month::May => 5,
            Month::Jun => 6,
            Month::Jul => 7,
            Month::Aug => 8,
            Month::Sep => 9,
            Month::Oct => 10,
            Month::Nov => 11,
            Month::Dec => 12,
        }
    }

    fn next(&self) -> Self {
        match self {
            Month::Jan => Month::Feb,
            Month::Feb => Month::Mar,
            Month::Mar => Month::Apr,
            Month::Apr => Month::May,
            Month::May => Month::Jun,
            Month::Jun => Month::Jul,
            Month::Jul => Month::Aug,
            Month::Aug => Month::Sep,
            Month::Sep => Month::Oct,
            Month::Oct => Month::Nov,
            Month::Nov => Month::Dec,
            Month::Dec => Month::Jan,
        }
    }

    fn days(&self, year: u32) -> u8 {
        match self {
            Month::Jan => 31,
            Month::Feb => {
                if is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            Month::Mar => 31,
            Month::Apr => 30,
            Month::May => 31,
            Month::Jun => 30,
            Month::Jul => 31,
            Month::Aug => 31,
            Month::Sep => 30,
            Month::Oct => 31,
            Month::Nov => 30,
            Month::Dec => 31,
        }
    }
}

struct Date {
    day: u8,
    month: Month,
    year: u32,
}

impl Date {
    fn next_week(&mut self) {
        self.day += 7;
        let month_days = self.month.days(self.year);
        if self.day > month_days {
            self.day -= month_days;
            let next_month = self.month.next();
            if next_month.num() < self.month.num() {
                self.year += 1;
            }
            self.month = next_month;
        }
    }
}

pub fn solve() -> usize {
    // first sunday in 1900
    let mut date = Date {
        day: 6,
        month: Month::Jan,
        year: 1901,
    };

    let mut sundays_on_1st = 0;
    while date.year <= 2000 {
        if date.day == 1 {
            sundays_on_1st += 1;
        }
        date.next_week()
    }
    sundays_on_1st
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(171, super::solve());
    }
}
