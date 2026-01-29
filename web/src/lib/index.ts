const days = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday']
const months = [
  'January',
  'February',
  'March',
  'April',
  'May',
  'June',
  'July',
  'August',
  'September',
  'October',
  'November',
  'December',
]

export function epochToDateString(epoch: number) {
  const date = new Date(0)
  date.setUTCSeconds(epoch)

  return {
    dayString: days[date.getDay()].slice(0, 3),
    month: months[date.getMonth()].slice(0, 3),
    day: date.getDate(),
  }
}
