import type { NextConfig } from 'next'

const nextConfig: NextConfig = {
  images: {
    remotePatterns: [
      {
        protocol: 'http',
        hostname: 'cdn.weatherapi.com',
        port: '',
      },
    ],
  },
}

export default nextConfig
