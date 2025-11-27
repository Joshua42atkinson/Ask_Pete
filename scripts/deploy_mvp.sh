#!/bin/bash
# Exit immediately if a command exits with a non-zero status.
set -e

# Build the Docker image
gcloud builds submit --tag gcr.io/$GOOGLE_CLOUD_PROJECT/ask-pete-mvp

# Deploy to Cloud Run
gcloud run deploy ask-pete-mvp \
  --image gcr.io/$GOOGLE_CLOUD_PROJECT/ask-pete-mvp \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated \
  --set-env-vars LEPTOS_SITE_ROOT=site
