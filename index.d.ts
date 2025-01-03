/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum HttpMethod {
  GET = 'GET',
  POST = 'POST',
  PUT = 'PUT',
  DELETE = 'DELETE',
  PATCH = 'PATCH',
  HEAD = 'HEAD',
  OPTIONS = 'OPTIONS'
}
export interface RequestInit {
  method?: HttpMethod
  headers?: Record<string, string>
  body?: string | Buffer
  /** Request timeout in milliseconds. Overrides the Retcher-wide timeout option. */
  timeout?: number
  /** Force the request to use HTTP/3. If the server doesn't expect HTTP/3, the request will fail. */
  forceHttp3?: boolean
}
export const enum Browser {
  Chrome = 'Chrome',
  Firefox = 'Firefox'
}
export interface RetcherOptions {
  browser?: Browser
  ignoreTlsErrors?: boolean
  vanillaFallback?: boolean
  proxyUrl?: string
  /** Default timeout for this Retcher instance in milliseconds. */
  timeout?: number
  /** Enable HTTP/3 support. */
  http3?: boolean
  /** Follow redirects. */
  followRedirects?: boolean
  /**
   * Maximum number of redirects to follow. Default is `10`.
   *
   * If this number is exceeded, the request will be rejected with an error.
   */
  maxRedirects?: number
}
export declare class RetchResponse {
  status: number
  statusText: string
  headers: Record<string, string>
  ok: boolean
  bytes(): Buffer
  text(): string
  json(): any
}
export type RetcherWrapper = Retcher
export declare class Retcher {
  constructor(options?: RetcherOptions | undefined | null)
  fetch(url: string, requestInit?: RequestInit | undefined | null): Promise<RetchResponse>
}
