import { FaCloudRain } from 'react-icons/fa'

// Main content of the weather component
function WeatherMain() {
  return (
    <div className="flex flex-row">
      <div className="flex w-1/2 items-center">
        <FaCloudRain size={96} />
      </div>
      <div className="w-1/2 flex flex-col items-center">
        <h1 className="text-6xl">45°</h1>
        <h2 className="text-3xl">City, State</h2>
        <h3 className="text-xl">Cloudy</h3>
      </div>
    </div>
  )
}

// Footer of the weather component, to show weather for the next 5 days
function WeatherFooter() {
  return (
    <div className="flex flex-row gap-4">
      <WeatherCard />
      <WeatherCard />
      <WeatherCard />
      <WeatherCard />
      <WeatherCard />
    </div>
  )
}

function WeatherCard() {
  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-row">
        <div className="flex w-1/4 items-center">
          <FaCloudRain size={32} />
        </div>
        <div className="w-1/2 flex flex-col items-center">
          <h1 className="text-lg">Mon</h1>
          <h2 className="text-md">Jan 1</h2>
        </div>
      </div>
      <div className="flex flex-row gap-2 w-full">
        <h1>
          <b>H:</b> 100°
        </h1>
        <h1>
          <b>L:</b> 20°
        </h1>
      </div>
    </div>
  )
}

export default function Weather() {
  return (
    <div className="flex flex-col w-full h-full gap-8 m-6">
      <WeatherMain />
      <WeatherFooter />
    </div>
  )
}
