import React, { useState, useEffect } from 'react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Activity, Shield, Zap, Users } from 'lucide-react'

interface Stats {
  total_queries: number
  blocked_threats: number
  cache_hits: number
  active_users: number
  avg_response_time: number
}

export function LiveStats() {
  const [stats, setStats] = useState<Stats>({
    total_queries: 0,
    blocked_threats: 0,
    cache_hits: 0,
    active_users: 0,
    avg_response_time: 0
  })

  useEffect(() => {
    // Simulate real-time updates
    const interval = setInterval(() => {
      setStats(prev => ({
        total_queries: prev.total_queries + Math.floor(Math.random() * 5),
        blocked_threats: prev.blocked_threats + (Math.random() > 0.8 ? 1 : 0),
        cache_hits: prev.cache_hits + Math.floor(Math.random() * 3),
        active_users: 42 + Math.floor(Math.random() * 10),
        avg_response_time: 15 + Math.floor(Math.random() * 20)
      }))
    }, 2000)

    return () => clearInterval(interval)
  }, [])

  const statCards = [
    {
      title: "Total Queries",
      value: stats.total_queries.toLocaleString(),
      description: "DNS queries processed",
      icon: Activity,
      trend: "+12%"
    },
    {
      title: "Threats Blocked",
      value: stats.blocked_threats.toLocaleString(),
      description: "Malicious domains blocked",
      icon: Shield,
      trend: "+5%"
    },
    {
      title: "Cache Hits",
      value: stats.cache_hits.toLocaleString(),
      description: "Queries served from cache",
      icon: Zap,
      trend: "+23%"
    },
    {
      title: "Active Users",
      value: stats.active_users.toString(),
      description: "Connected clients",
      icon: Users,
      trend: "+8%"
    }
  ]

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {statCards.map((stat, index) => {
          const Icon = stat.icon
          return (
            <Card key={index} className="relative overflow-hidden">
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">
                  {stat.title}
                </CardTitle>
                <Icon className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">{stat.value}</div>
                <p className="text-xs text-muted-foreground">
                  {stat.description}
                </p>
                <div className="flex items-center text-xs text-green-600 dark:text-green-400 mt-1">
                  <span className="font-medium">{stat.trend}</span>
                  <span className="ml-1">from last hour</span>
                </div>
              </CardContent>
            </Card>
          )
        })}
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Performance Metrics</CardTitle>
          <CardDescription>Real-time DNS server performance</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <span className="text-sm font-medium">Average Response Time</span>
              <span className="text-sm text-muted-foreground">{stats.avg_response_time}ms</span>
            </div>
            <div className="w-full bg-secondary rounded-full h-2">
              <div 
                className="bg-primary h-2 rounded-full transition-all duration-500" 
                style={{ width: `${Math.max(10, 100 - stats.avg_response_time)}%` }}
              />
            </div>
            
            <div className="grid grid-cols-2 gap-4 mt-4">
              <div className="text-center">
                <div className="text-2xl font-bold text-green-600">99.9%</div>
                <div className="text-sm text-muted-foreground">Uptime</div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-blue-600">85%</div>
                <div className="text-sm text-muted-foreground">Cache Hit Rate</div>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}