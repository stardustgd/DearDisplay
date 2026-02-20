'use client'

export default function DateComponent() {
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

  const date = new Date()

  return (
    <div>
      <h1 className="font-semibold text-3xl">
        {months[date.getMonth()]} {date.getDate()}, {date.getFullYear()}
      </h1>
    </div>
  )
}
