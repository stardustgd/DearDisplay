import { WeatherResponse, Location, CurrentWeather, ForecastDay } from '@/types'
import Image from 'next/image'
import { epochToDateString, getWeatherIcon } from '@/lib'

type WeatherMainProps = {
  location: Location
  data: CurrentWeather
}

type WeatherFooterProps = {
  data: ForecastDay[]
}

type WeatherCardProps = {
  data: ForecastDay
}

// Main content of the weather component
function WeatherMain({ location, data }: WeatherMainProps) {
  const condition = data.condition

  return (
    <div className="flex flex-row">
      <div className="flex w-1/4 items-center">
        <Image src={getWeatherIcon(condition.icon)} alt={condition.text} width={128} height={128} />
      </div>
      <div className="w-1/2 flex flex-col items-center">
        <h1 className="text-6xl">{data.temp_f}°</h1>
        <h2 className="text-3xl">
          {location.name}, {location.region}
        </h2>
        <h3 className="text-xl">{data.condition.text}</h3>
      </div>
    </div>
  )
}

// Footer of the weather component, to show weather for the next 5 days
function WeatherFooter({ data }: WeatherFooterProps) {
  return (
    <div className="flex flex-row w-full">
      {data.slice(1).map((day: ForecastDay, index) => (
        <WeatherCard data={day} key={index} />
      ))}
    </div>
  )
}

function WeatherCard({ data }: WeatherCardProps) {
  const condition = data.day.condition
  const date = epochToDateString(data.date_epoch)

  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-row">
        <div className="flex w-1/4 items-center">
          <Image src={getWeatherIcon(condition.icon)} alt={condition.text} width={128} height={128} />
        </div>
        <div className="w-1/2 flex flex-col items-center">
          <h1 className="text-lg font-bold">{date.dayString}</h1>
          <h2 className="text-md">
            {date.month} {date.day}
          </h2>
        </div>
      </div>
      <div className="flex flex-row gap-2 w-full">
        <h1>
          <b>H:</b> {Math.round(data.day.maxtemp_f)}°
        </h1>
        <h1>
          <b>L:</b> {Math.round(data.day.mintemp_f)}°
        </h1>
      </div>
    </div>
  )
}

export default async function Weather() {
  const data = await fetch(
    `http://api.weatherapi.com/v1/forecast.json?key=${process.env.WEATHER_API_KEY}&q=${process.env.WEATHER_Q}&days=6`
  )
  const weather: WeatherResponse = await data.json()

  return (
    <div className="flex flex-col w-full h-full gap-8 m-6">
      <WeatherMain location={weather.location} data={weather.current} />
      <WeatherFooter data={weather.forecast.forecastday} />
    </div>
  )
}
