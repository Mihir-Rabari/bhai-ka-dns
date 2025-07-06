import React, { useState } from 'react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Loader2, Search, Shield, AlertTriangle, CheckCircle } from 'lucide-react'

interface DNSResult {
  domain: string
  ip: string
  security_score: number
  threat_level: string
  response_time: number
}

export function DNSLookupTool() {
  const [domain, setDomain] = useState('')
  const [result, setResult] = useState<DNSResult | null>(null)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState('')

  const handleLookup = async () => {
    if (!domain.trim()) return

    setIsLoading(true)
    setError('')
    setResult(null)

    try {
      const response = await fetch('/api/lookup', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ domain: domain.trim() }),
      })

      if (!response.ok) {
        throw new Error('Lookup failed')
      }

      const data = await response.json()
      setResult(data)
    } catch (err) {
      setError('Failed to lookup domain. Please try again.')
    } finally {
      setIsLoading(false)
    }
  }

  const getSecurityIcon = (score: number) => {
    if (score >= 80) return <CheckCircle className="h-4 w-4 text-green-500" />
    if (score >= 60) return <Shield className="h-4 w-4 text-yellow-500" />
    return <AlertTriangle className="h-4 w-4 text-red-500" />
  }

  const getSecurityColor = (score: number) => {
    if (score >= 80) return 'text-green-500'
    if (score >= 60) return 'text-yellow-500'
    return 'text-red-500'
  }

  return (
    <Card className="w-full max-w-2xl mx-auto">
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Search className="h-5 w-5" />
          DNS Lookup Tool
        </CardTitle>
        <CardDescription>
          Enter a domain to perform AI-powered DNS analysis
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="flex gap-2">
          <Input
            placeholder="Enter domain (e.g., google.com)"
            value={domain}
            onChange={(e) => setDomain(e.target.value)}
            onKeyPress={(e) => e.key === 'Enter' && handleLookup()}
            className="flex-1"
          />
          <Button onClick={handleLookup} disabled={isLoading || !domain.trim()}>
            {isLoading ? (
              <Loader2 className="h-4 w-4 animate-spin" />
            ) : (
              <Search className="h-4 w-4" />
            )}
            Lookup
          </Button>
        </div>

        {error && (
          <div className="p-3 rounded-md bg-red-50 dark:bg-red-950 text-red-600 dark:text-red-400 text-sm">
            {error}
          </div>
        )}

        {result && (
          <div className="space-y-4 p-4 rounded-lg bg-muted/50">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label className="text-sm font-medium text-muted-foreground">Domain</label>
                <p className="font-mono text-sm">{result.domain}</p>
              </div>
              <div>
                <label className="text-sm font-medium text-muted-foreground">IP Address</label>
                <p className="font-mono text-sm">{result.ip}</p>
              </div>
              <div>
                <label className="text-sm font-medium text-muted-foreground">Response Time</label>
                <p className="text-sm">{result.response_time}ms</p>
              </div>
              <div>
                <label className="text-sm font-medium text-muted-foreground">Security Analysis</label>
                <div className="flex items-center gap-2">
                  {getSecurityIcon(result.security_score)}
                  <span className={`text-sm font-medium ${getSecurityColor(result.security_score)}`}>
                    {result.security_score}/100 ({result.threat_level})
                  </span>
                </div>
              </div>
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  )
}