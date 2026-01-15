class APIManager {
  serverPort: number | null
  constructor(serverPort: number | null) {
    this.serverPort = serverPort
  }
  async post(endpoint: string, data: any): Promise<any> {
    try {
      const response = await fetch(endpoint, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
      })
      if (!response.ok) {
        throw new Error(`API request failed with status ${response.status}`)
      }
      return await response.json()
    } catch (error) {
      console.error('API request error:', error)
      throw error
    }
  }
  async get(endpoint: string): Promise<any> {
    try {
      const response = await fetch(endpoint)
      if (!response.ok) {
        throw new Error(`API request failed with status ${response.status}`)
      }
      return await response.json()
    } catch (error) {
      console.error('API request error:', error)
      throw error
    }
  }
}

export const apiManager = new APIManager(null)
