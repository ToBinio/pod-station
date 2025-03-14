import type { UseFetchOptions } from '#app'
import type { FetchError } from 'ofetch'

interface CustomError {
  message: string
  statusCode: number
}

export function useApiFetch<T>(
  url: string | (() => string),
  options?: UseFetchOptions<T>,
) {
  //todo fix ts
  //@ts-ignore
  return useFetch<T, FetchError<CustomError>>(url, {
    ...options,
    baseURL: useRuntimeConfig().public.baseURL
  })
}
