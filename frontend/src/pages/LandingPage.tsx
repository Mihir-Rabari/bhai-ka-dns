import { motion } from 'framer-motion'
import { useSpring, animated } from 'react-spring'
import { useState, useEffect } from 'react'
import { Shield, Zap, Brain, Globe, ArrowRight, Play } from 'lucide-react'

import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { DNSLookupTool } from '@/components/features/DNSLookupTool'
import { LiveStats } from '@/components/features/LiveStats'
import { FeatureShowcase } from '@/components/features/FeatureShowcase'

export function LandingPage() {
  const [mounted, setMounted] = useState(false)
  
  useEffect(() => {
    setMounted(true)
  }, [])

  const heroAnimation = useSpring({
    opacity: mounted ? 1 : 0,
    transform: mounted ? 'translateY(0px)' : 'translateY(50px)',
    config: { tension: 280, friction: 60 }
  })

  const statsAnimation = useSpring({
    opacity: mounted ? 1 : 0,
    transform: mounted ? 'scale(1)' : 'scale(0.8)',
    delay: 200,
    config: { tension: 200, friction: 50 }
  })

  return (
    <div className="space-y-16">
      {/* Hero Section */}
      <animated.section style={heroAnimation} className="text-center space-y-8 py-20">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6 }}
          className="space-y-4"
        >
          <h1 className="text-6xl font-bold gradient-text leading-tight">
            Bhai Ka DNS
          </h1>
          <p className="text-2xl text-muted-foreground">
            AI-Powered DNS with Next-Gen Security
          </p>
          <p className="text-lg text-muted-foreground max-w-2xl mx-auto">
            Experience lightning-fast DNS resolution powered by advanced AI algorithms 
            for threat detection, smart caching, and intelligent domain analysis.
          </p>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6, delay: 0.2 }}
          className="flex justify-center space-x-4"
        >
          <Button size="lg" className="group">
            Get Started
            <ArrowRight className="ml-2 h-4 w-4 group-hover:translate-x-1 transition-transform" />
          </Button>
          <Button variant="outline" size="lg" className="group">
            <Play className="mr-2 h-4 w-4" />
            Watch Demo
          </Button>
        </motion.div>

        {/* Animated Background Elements */}
        <div className="absolute inset-0 -z-10 overflow-hidden">
          {[...Array(20)].map((_, i) => (
            <motion.div
              key={i}
              className="absolute w-2 h-2 bg-blue-500/20 rounded-full"
              animate={{
                x: [0, 100, 0],
                y: [0, -100, 0],
                opacity: [0, 1, 0],
              }}
              transition={{
                duration: 3 + i * 0.2,
                repeat: Infinity,
                delay: i * 0.1,
              }}
              style={{
                left: `${Math.random() * 100}%`,
                top: `${Math.random() * 100}%`,
              }}
            />
          ))}
        </div>
      </animated.section>

      {/* Live Stats */}
      <animated.section style={statsAnimation}>
        <LiveStats />
      </animated.section>

      {/* DNS Lookup Tool */}
      <motion.section
        initial={{ opacity: 0, y: 50 }}
        whileInView={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.6 }}
        viewport={{ once: true }}
      >
        <DNSLookupTool />
      </motion.section>

      {/* Features Grid */}
      <motion.section
        initial={{ opacity: 0 }}
        whileInView={{ opacity: 1 }}
        transition={{ duration: 0.8 }}
        viewport={{ once: true }}
        className="space-y-12"
      >
        <div className="text-center space-y-4">
          <h2 className="text-4xl font-bold">Why Choose Bhai Ka DNS?</h2>
          <p className="text-xl text-muted-foreground">
            Cutting-edge features powered by artificial intelligence
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          {features.map((feature, index) => (
            <motion.div
              key={feature.title}
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5, delay: index * 0.1 }}
              viewport={{ once: true }}
            >
              <Card className="glass-morphism hover:glow-blue transition-all duration-300 group">
                <CardHeader>
                  <div className="w-12 h-12 bg-gradient-to-r from-blue-500 to-purple-600 rounded-lg flex items-center justify-center mb-4 group-hover:scale-110 transition-transform">
                    <feature.icon className="h-6 w-6 text-white" />
                  </div>
                  <CardTitle>{feature.title}</CardTitle>
                  <CardDescription>{feature.description}</CardDescription>
                </CardHeader>
              </Card>
            </motion.div>
          ))}
        </div>
      </motion.section>

      {/* Feature Showcase */}
      <FeatureShowcase />

      {/* CTA Section */}
      <motion.section
        initial={{ opacity: 0, scale: 0.9 }}
        whileInView={{ opacity: 1, scale: 1 }}
        transition={{ duration: 0.6 }}
        viewport={{ once: true }}
        className="text-center space-y-8 py-20"
      >
        <Card className="glass-morphism max-w-4xl mx-auto">
          <CardContent className="pt-12 pb-16 space-y-6">
            <h2 className="text-4xl font-bold gradient-text">
              Ready to Experience the Future of DNS?
            </h2>
            <p className="text-xl text-muted-foreground max-w-2xl mx-auto">
              Join thousands of users who trust Bhai Ka DNS for secure, 
              fast, and intelligent domain resolution.
            </p>
            <div className="flex justify-center space-x-4">
              <Button size="lg" className="animate-pulse-glow">
                Start Free Trial
              </Button>
              <Button variant="outline" size="lg">
                View Documentation
              </Button>
            </div>
          </CardContent>
        </Card>
      </motion.section>
    </div>
  )
}

const features = [
  {
    icon: Shield,
    title: "AI Threat Detection",
    description: "Advanced machine learning algorithms detect and block malicious domains in real-time."
  },
  {
    icon: Zap,
    title: "Lightning Fast",
    description: "Intelligent caching and optimization deliver sub-millisecond response times."
  },
  {
    icon: Brain,
    title: "Smart Analysis",
    description: "Comprehensive domain analysis with security scoring and recommendations."
  },
  {
    icon: Globe,
    title: "Global Network",
    description: "Distributed infrastructure ensures optimal performance worldwide."
  }
]