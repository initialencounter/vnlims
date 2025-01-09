import type { DataModel } from '../types'

export interface TrackData {
  total: number
  rows: DataModel[]
}
export async function projectTracking(query: string): Promise<DataModel[]> {
  const url = `${window.location.origin}/rest/project?${query}`
  const response = await fetch(url)
  const data: TrackData = await response.json()
  if (!response.ok) {
    return []
  }
  return data.rows
}

export function isDev() {
  // @ts-ignore
  return process.env.NODE_ENV === 'development'
}
