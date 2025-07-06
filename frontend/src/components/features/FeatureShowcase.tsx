import React from 'react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { 
  Shield, 
  Zap, 
  Brain, 
  Globe, 
  BarChart3, 
  Lock,
  RefreshCw,
  Smartphone
} from 'lucide-react'

export function FeatureShowcase() {
  const features = [
    {
      icon: Brain,
      title: "AI-Powered Threat Detection",
      description: "Advanced machine learning algorithms detect and block malicious domains in real-time with 99.9% accuracy.",
      tags: ["Machine Learning", "Real-time", "Security"],
      highlight: true
    },
    {
      icon: Zap,
      title: "Lightning Fast Performance",
      description: "Sub-millisecond response times with intelligent caching and optimized DNS resolution.",
      tags: ["Performance", "Caching", "Speed"],
      highlight: false
    },
    {
      icon: Shield,
      title: "Enterprise Security",
      description: "Military-grade encryption, DDoS protection, and comprehensive threat intelligence feeds.",
      tags: ["Encryption", "DDoS Protection", "Enterprise"],
      highlight: false
    },
    {
      icon: Globe,
      title: "Global CDN Network",
      description: "Worldwide edge locations ensure minimal latency and maximum availability for your users.",
      tags: ["CDN", "Global", "Availability"],
      highlight: false
    },
    {
      icon: BarChart3,
      title: "Real-time Analytics",
      description: "Comprehensive dashboards with detailed insights into DNS queries, threats, and performance metrics.",
      tags: ["Analytics", "Dashboards", "Insights"],
      highlight: false
    },
    {
      icon: RefreshCw,
      title: "Auto-scaling Infrastructure",
      description: "Kubernetes-based architecture that automatically scales based on demand using AWS EKS.",
      tags: ["Kubernetes", "Auto-scaling", "AWS"],
      highlight: true
    },
    {
      icon: Lock,
      title: "Zero-Trust Security",
      description: "Every query is analyzed and verified against multiple threat intelligence sources.",
      tags: ["Zero-Trust", "Verification", "Intelligence"],
      highlight: false
    },
    {
      icon: Smartphone,
      title: "Multi-Platform Support",
      description: "Works seamlessly across all devices and platforms with native mobile applications.",
      tags: ["Mobile", "Cross-platform", "Native"],
      highlight: false
    }
  ]

  return (
    <div className="space-y-8">
      <div className="text-center space-y-4">
        <h2 className="text-3xl font-bold tracking-tight">
          Powerful Features for Modern DNS
        </h2>
        <p className="text-xl text-muted-foreground max-w-2xl mx-auto">
          Experience the next generation of DNS services with AI-powered security,
          global performance, and enterprise-grade reliability.
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {features.map((feature, index) => {
          const Icon = feature.icon
          return (
            <Card 
              key={index} 
              className={`relative transition-all duration-300 hover:shadow-lg ${
                feature.highlight 
                  ? 'ring-2 ring-primary/20 bg-gradient-to-br from-primary/5 to-transparent' 
                  : 'hover:shadow-md'
              }`}
            >
              {feature.highlight && (
                <div className="absolute -top-2 -right-2">
                  <Badge variant="default" className="shadow-md">
                    Popular
                  </Badge>
                </div>
              )}
              
              <CardHeader>
                <div className="flex items-center space-x-3">
                  <div className={`p-2 rounded-lg ${
                    feature.highlight 
                      ? 'bg-primary text-primary-foreground' 
                      : 'bg-muted'
                  }`}>
                    <Icon className="h-5 w-5" />
                  </div>
                  <CardTitle className="text-lg">{feature.title}</CardTitle>
                </div>
              </CardHeader>
              
              <CardContent className="space-y-4">
                <CardDescription className="text-sm leading-relaxed">
                  {feature.description}
                </CardDescription>
                
                <div className="flex flex-wrap gap-1">
                  {feature.tags.map((tag, tagIndex) => (
                    <Badge 
                      key={tagIndex} 
                      variant="secondary" 
                      className="text-xs"
                    >
                      {tag}
                    </Badge>
                  ))}
                </div>
              </CardContent>
            </Card>
          )
        })}
      </div>

      <div className="bg-muted/50 rounded-lg p-8 space-y-6">
        <div className="text-center space-y-2">
          <h3 className="text-2xl font-bold">Ready to Get Started?</h3>
          <p className="text-muted-foreground">
            Deploy Bhai Ka DNS in minutes with our comprehensive CLI tool
          </p>
        </div>
        
        <div className="bg-background rounded-lg p-4 font-mono text-sm">
          <div className="text-muted-foreground mb-2"># Install the CLI tool</div>
          <div className="text-primary">curl -sSL https://install.bhai-dns.com | bash</div>
          <div className="text-muted-foreground mt-2"># Initialize your project</div>
          <div className="text-primary">bhai-dns-cli init --project-name my-dns</div>
          <div className="text-muted-foreground mt-2"># Start locally</div>
          <div className="text-primary">bhai-dns-cli dev start</div>
        </div>
      </div>
    </div>
  )
}