#!/bin/bash

# Bhai Ka DNS - Kubernetes Deployment Script
# This script deploys Bhai Ka DNS to a Kubernetes cluster

set -e

echo "☸️ Deploying Bhai Ka DNS to Kubernetes"
echo "======================================"

# Check prerequisites
check_prerequisites() {
    echo "📋 Checking prerequisites..."
    
    # Check kubectl
    if ! command -v kubectl &> /dev/null; then
        echo "❌ kubectl is required but not installed. Please install kubectl first."
        exit 1
    fi
    
    # Check if cluster is accessible
    if ! kubectl cluster-info &> /dev/null; then
        echo "❌ Cannot connect to Kubernetes cluster. Please check your kubectl configuration."
        exit 1
    fi
    
    echo "✅ Prerequisites check passed!"
}

# Build and push Docker images
build_and_push_images() {
    echo "🏗️ Building and pushing Docker images..."
    
    # Get registry from environment or use default
    REGISTRY=${DOCKER_REGISTRY:-"localhost:5000"}
    
    # Build backend image
    echo "Building backend image..."
    docker build -t ${REGISTRY}/bhai-dns-backend:latest .
    docker push ${REGISTRY}/bhai-dns-backend:latest
    
    # Build frontend image
    echo "Building frontend image..."
    docker build -t ${REGISTRY}/bhai-dns-frontend:latest ./frontend
    docker push ${REGISTRY}/bhai-dns-frontend:latest
    
    echo "✅ Images built and pushed successfully!"
}

# Update image references in manifests
update_manifests() {
    echo "📝 Updating Kubernetes manifests..."
    
    REGISTRY=${DOCKER_REGISTRY:-"localhost:5000"}
    
    # Update backend deployment
    sed -i "s|bhai-dns-backend:latest|${REGISTRY}/bhai-dns-backend:latest|g" k8s/backend.yaml
    
    echo "✅ Manifests updated!"
}

# Deploy to Kubernetes
deploy_to_kubernetes() {
    echo "🚀 Deploying to Kubernetes..."
    
    # Create namespace
    echo "Creating namespace..."
    kubectl apply -f k8s/namespace.yaml
    
    # Apply ConfigMap
    echo "Applying ConfigMap..."
    kubectl apply -f k8s/configmap.yaml
    
    # Deploy MongoDB
    echo "Deploying MongoDB..."
    kubectl apply -f k8s/mongodb.yaml
    
    # Wait for MongoDB to be ready
    echo "Waiting for MongoDB to be ready..."
    kubectl wait --for=condition=available --timeout=300s deployment/mongodb -n bhai-dns
    
    # Deploy backend
    echo "Deploying backend..."
    kubectl apply -f k8s/backend.yaml
    
    # Wait for backend to be ready
    echo "Waiting for backend to be ready..."
    kubectl wait --for=condition=available --timeout=300s deployment/bhai-dns-backend -n bhai-dns
    
    echo "✅ Deployment complete!"
}

# Verify deployment
verify_deployment() {
    echo "🔍 Verifying deployment..."
    
    # Check pod status
    echo "Pod status:"
    kubectl get pods -n bhai-dns
    
    # Check service status
    echo "Service status:"
    kubectl get services -n bhai-dns
    
    # Check if backend is responding
    echo "Testing backend health..."
    kubectl port-forward -n bhai-dns svc/bhai-dns-backend-api 8080:8080 &
    PORT_FORWARD_PID=$!
    sleep 5
    
    if curl -f -s http://localhost:8080/health > /dev/null; then
        echo "✅ Backend is responding"
    else
        echo "❌ Backend is not responding"
    fi
    
    kill $PORT_FORWARD_PID 2>/dev/null || true
}

# Show access information
show_access_info() {
    echo ""
    echo "🎉 Kubernetes deployment complete!"
    echo "================================="
    echo ""
    echo "🔧 Useful Commands:"
    echo "  View pods:        kubectl get pods -n bhai-dns"
    echo "  View services:    kubectl get services -n bhai-dns"
    echo "  View logs:        kubectl logs -f deployment/bhai-dns-backend -n bhai-dns"
    echo "  Port forward API: kubectl port-forward -n bhai-dns svc/bhai-dns-backend-api 8080:8080"
    echo "  Port forward DNS: kubectl port-forward -n bhai-dns svc/bhai-dns-backend-dns 5353:5353"
    echo ""
    echo "🌐 Access (after port forwarding):"
    echo "  API:       http://localhost:8080"
    echo "  DNS:       localhost:5353"
    echo ""
    echo "📊 Monitoring:"
    echo "  Setup Prometheus: kubectl apply -f monitoring/k8s/"
    echo "  Setup Grafana:    kubectl apply -f monitoring/k8s/"
    echo ""
}

# Cleanup deployment
cleanup_deployment() {
    echo "🧹 Cleaning up deployment..."
    
    kubectl delete -f k8s/backend.yaml || true
    kubectl delete -f k8s/mongodb.yaml || true
    kubectl delete -f k8s/configmap.yaml || true
    kubectl delete -f k8s/namespace.yaml || true
    
    echo "✅ Cleanup complete!"
}

# Main execution
main() {
    check_prerequisites
    
    if [ "${SKIP_BUILD:-false}" != "true" ]; then
        build_and_push_images
    fi
    
    update_manifests
    deploy_to_kubernetes
    verify_deployment
    show_access_info
}

# Parse command line arguments
case "${1:-}" in
    "clean")
        cleanup_deployment
        ;;
    "logs")
        kubectl logs -f deployment/bhai-dns-backend -n bhai-dns
        ;;
    "status")
        kubectl get all -n bhai-dns
        ;;
    "port-forward")
        echo "Setting up port forwarding..."
        kubectl port-forward -n bhai-dns svc/bhai-dns-backend-api 8080:8080 &
        kubectl port-forward -n bhai-dns svc/bhai-dns-backend-dns 5353:5353 &
        echo "✅ Port forwarding active. Press Ctrl+C to stop."
        wait
        ;;
    *)
        main
        ;;
esac