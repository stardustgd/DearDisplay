import type { Metadata } from 'next'
import { Ubuntu_Mono } from 'next/font/google'
import './globals.css'

const ubuntuMono = Ubuntu_Mono({
  weight: '400',
  variable: '--font-ubuntu-mono',
  subsets: ['latin'],
})

export const metadata: Metadata = {
  title: 'DearDisplay',
  description: 'E-ink display',
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="en">
      <body className={`${ubuntuMono.variable} antialiased`}>{children}</body>
    </html>
  )
}
