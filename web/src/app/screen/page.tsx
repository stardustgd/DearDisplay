import DateComponent from '@/components/screens/Date'
import Message from '@/components/screens/Message'
import Weather from '@/components/screens/Weather'

export const dynamic = 'force-dynamic'

export default function ScreenPage() {
  return (
    <div className="flex flex-col mx-8 my-6 gap-4 h-full">
      <DateComponent />
      <div className="border border-black">
        <Weather />
      </div>
      <div className="border border-black">
        <Message />
      </div>
    </div>
  )
}
