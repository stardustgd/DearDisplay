FROM node:25-bookworm-slim AS base
WORKDIR /usr/src/app

FROM base AS deps
WORKDIR /usr/src/app
COPY package.json package-lock.json ./
RUN npm ci

FROM base AS build
WORKDIR /usr/src/app
COPY --from=deps /usr/src/app/node_modules ./node_modules
COPY . ./
RUN npm run build

FROM base AS runner
WORKDIR /usr/src/app
COPY --from=build /usr/src/app/.next/standalone ./
COPY --from=build /usr/src/app/.next/static ./.next/static
COPY --from=build /usr/src/app/public ./public
USER nobody
EXPOSE 3000
CMD ["node", "server.js"]
