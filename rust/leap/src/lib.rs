// on every year that is evenly divisible by 4
// except every year that is evenly divisible by 100
// unless the year is also evenly divisible by 400
pub fn is_leap_year(year: u64) -> bool {
    if year % 400 == 0 {
        return true;
    }
    if year % 100 != 0 && year % 4 == 0 {
        return true;
    }

    return false;
}
